/**
 * WebGPU Renderer for 3D Viewer
 * This module handles GPU-accelerated rendering using WebGPU
 */

export class WebGPURenderer {
    constructor() {
        this.device = null;
        this.context = null;
        this.renderPipeline = null;
        this.uniformBuffer = null;
        this.vertexBuffer = null;
        this.indexBuffer = null;
        this.indexFormat = 'uint16'; // Default to 16-bit, will be updated based on data
        this.textureBindGroup = null;
        this.canvas = null;
        this.isInitialized = false;
    }

    async initialize(canvas) {
        this.canvas = canvas;
        
        // Check WebGPU support
        if (!navigator.gpu) {
            throw new Error('WebGPU is not supported in this browser');
        }

        // Request adapter and device
        const adapter = await navigator.gpu.requestAdapter();
        if (!adapter) {
            throw new Error('No appropriate GPUAdapter found');
        }

        this.device = await adapter.requestDevice();
        this.context = canvas.getContext('webgpu');
        
        const canvasFormat = navigator.gpu.getPreferredCanvasFormat();
        this.context.configure({
            device: this.device,
            format: canvasFormat,
        });

        await this.createShaders();
        await this.createBuffers();
        await this.createBindGroups();
        
        this.isInitialized = true;
    }

    async createShaders() {
        const vertexShaderSource = `
            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) texCoord: vec2<f32>,
            }

            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) worldPos: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) texCoord: vec2<f32>,
            }

            struct Uniforms {
                viewProjectionMatrix: mat4x4<f32>,
                modelMatrix: mat4x4<f32>,
                normalMatrix: mat3x4<f32>, // 3x4 to match memory layout
                cameraPosition: vec3<f32>,
            }

            @group(0) @binding(0) var<uniform> uniforms: Uniforms;

            @vertex
            fn vs_main(input: VertexInput) -> VertexOutput {
                var output: VertexOutput;
                
                // Scale down large vertex coordinates
                let scaledPosition = input.position * 0.001;
                
                let worldPos = uniforms.modelMatrix * vec4<f32>(scaledPosition, 1.0);
                output.worldPos = worldPos.xyz;
                output.position = uniforms.viewProjectionMatrix * worldPos;
                
                // Transform normals with the normal matrix
                let normalMatrix = mat3x3<f32>(
                    vec3<f32>(uniforms.normalMatrix[0].x, uniforms.normalMatrix[1].x, uniforms.normalMatrix[2].x),
                    vec3<f32>(uniforms.normalMatrix[0].y, uniforms.normalMatrix[1].y, uniforms.normalMatrix[2].y),
                    vec3<f32>(uniforms.normalMatrix[0].z, uniforms.normalMatrix[1].z, uniforms.normalMatrix[2].z)
                );
                
                let transformedNormal = normalMatrix * input.normal;
                output.normal = normalize(transformedNormal);
                output.texCoord = input.texCoord;
                
                return output;
            }
        `;

        const fragmentShaderSource = `
            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) worldPos: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) texCoord: vec2<f32>,
            }

            struct Uniforms {
                viewProjectionMatrix: mat4x4<f32>,
                modelMatrix: mat4x4<f32>,
                normalMatrix: mat3x4<f32>, // 3x4 to match memory layout
                cameraPosition: vec3<f32>,
            }

            @group(0) @binding(0) var<uniform> uniforms: Uniforms;
            @group(0) @binding(1) var diffuseTexture: texture_2d<f32>;
            @group(0) @binding(2) var normalTexture: texture_2d<f32>;
            @group(0) @binding(3) var specularTexture: texture_2d<f32>;
            @group(0) @binding(4) var emissionTexture: texture_2d<f32>;
            @group(0) @binding(5) var textureSampler: sampler;

            @fragment
            fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                // Flip Y coordinate to match OpenGL texture coordinate system
                let texCoord = vec2<f32>(input.texCoord.x, 1.0 - input.texCoord.y);
                
                let diffuseColor = textureSample(diffuseTexture, textureSampler, texCoord);
                let normalMap = textureSample(normalTexture, textureSampler, texCoord);
                let specularColor = textureSample(specularTexture, textureSampler, texCoord);
                let emissionColor = textureSample(emissionTexture, textureSampler, texCoord);

                // Debug: Show texture coordinates as colors (uncomment to debug)
                // return vec4<f32>(texCoord.x, texCoord.y, 0.0, 1.0);
                
                // Debug: Show normals as colors (uncomment to debug)
                // return vec4<f32>(input.normal * 0.5 + 0.5, 1.0);
                
                let N = normalize(input.normal);

                // Basic lighting calculation
                let lightDir = normalize(vec3<f32>(1.0, 1.0, 1.0));
                let viewDir = normalize(uniforms.cameraPosition - input.worldPos);

                // Ambient component
                let ambient = vec3<f32>(0.2);
                
                // Diffuse component
                let diffuse = max(dot(N, lightDir), 0.0);
                
                // Specular component
                let halfwayDir = normalize(lightDir + viewDir);
                let specular = pow(max(dot(N, halfwayDir), 0.0), 32.0) * specularColor.r;

                // Combine lighting (simpler version for debugging)
                let finalColor = ambient * diffuseColor.rgb + 
                                diffuseColor.rgb * diffuse + 
                                vec3<f32>(specular) + 
                                emissionColor.rgb;
                
                return vec4<f32>(finalColor, diffuseColor.a);
            }
        `;

        const vertexShader = this.device.createShaderModule({
            label: 'Vertex Shader',
            code: vertexShaderSource,
        });

        const fragmentShader = this.device.createShaderModule({
            label: 'Fragment Shader',
            code: fragmentShaderSource,
        });

        this.renderPipeline = this.device.createRenderPipeline({
            label: 'Render Pipeline',
            layout: 'auto',
            vertex: {
                module: vertexShader,
                entryPoint: 'vs_main',
                buffers: [{
                    arrayStride: 8 * 4, // 3 position + 3 normal + 2 texCoord
                    attributes: [
                        {
                            shaderLocation: 0,
                            offset: 0,
                            format: 'float32x3',
                        },
                        {
                            shaderLocation: 1,
                            offset: 3 * 4,
                            format: 'float32x3',
                        },
                        {
                            shaderLocation: 2,
                            offset: 6 * 4,
                            format: 'float32x2',
                        },
                    ],
                }],
            },
            fragment: {
                module: fragmentShader,
                entryPoint: 'fs_main',
                targets: [{
                    format: navigator.gpu.getPreferredCanvasFormat(),
                }],
            },
            primitive: {
                topology: 'triangle-list',
                cullMode: 'back',
                frontFace: 'ccw',
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less-equal', // Try less-equal instead of less
                format: 'depth24plus',
            },
        });
    }

    async createBuffers() {
        // Create uniform buffer
        this.uniformBuffer = this.device.createBuffer({
            label: 'Uniform Buffer',
            size: 320, // 80 floats * 4 bytes per float
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        // Create depth texture
        this.depthTexture = this.device.createTexture({
            size: [this.canvas.width, this.canvas.height],
            format: 'depth24plus',
            usage: GPUTextureUsage.RENDER_ATTACHMENT,
        });
    }

    async createBindGroups() {
        // Create default textures (1x1 white/normal textures)
        const createDefaultTexture = (color) => {
            const texture = this.device.createTexture({
                size: [1, 1],
                format: 'rgba8unorm',
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
            });
            
            this.device.queue.writeTexture(
                { texture },
                new Uint8Array(color),
                { bytesPerRow: 4 },
                { width: 1, height: 1 }
            );
            
            return texture;
        };

        this.defaultDiffuseTexture = createDefaultTexture([255, 255, 255, 255]); // White
        this.defaultNormalTexture = createDefaultTexture([128, 128, 255, 255]); // Neutral normal (0,0,1 in normal space)
        this.defaultSpecularTexture = createDefaultTexture([128, 128, 128, 255]); // Medium gray
        this.defaultEmissionTexture = createDefaultTexture([0, 0, 0, 255]); // Black (no emission)

        const sampler = this.device.createSampler({
            magFilter: 'linear',
            minFilter: 'linear',
            addressModeU: 'repeat',
            addressModeV: 'repeat',
        });

        this.textureBindGroup = this.device.createBindGroup({
            label: 'Texture Bind Group',
            layout: this.renderPipeline.getBindGroupLayout(0),
            entries: [
                {
                    binding: 0,
                    resource: {
                        buffer: this.uniformBuffer,
                    },
                },
                {
                    binding: 1,
                    resource: this.defaultDiffuseTexture.createView(),
                },
                {
                    binding: 2,
                    resource: this.defaultNormalTexture.createView(),
                },
                {
                    binding: 3,
                    resource: this.defaultSpecularTexture.createView(),
                },
                {
                    binding: 4,
                    resource: this.defaultEmissionTexture.createView(),
                },
                {
                    binding: 5,
                    resource: sampler,
                },
            ],
        });
    }

    updateVertexData(vertices, indices) {
        if (!this.isInitialized) return;

        // Create or update vertex buffer
        if (this.vertexBuffer) {
            this.vertexBuffer.destroy();
        }
        
        this.vertexBuffer = this.device.createBuffer({
            label: 'Vertex Buffer',
            size: vertices.byteLength,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        
        this.device.queue.writeBuffer(this.vertexBuffer, 0, vertices);

        // Create or update index buffer
        if (this.indexBuffer) {
            this.indexBuffer.destroy();
        }
        
        this.indexBuffer = this.device.createBuffer({
            label: 'Index Buffer',
            size: indices.byteLength,
            usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST,
        });
        
        this.device.queue.writeBuffer(this.indexBuffer, 0, indices);
        this.indexCount = indices.length;
        
        // Determine index format based on the typed array type
        this.indexFormat = indices instanceof Uint32Array ? 'uint32' : 'uint16';
        console.log('Using index format:', this.indexFormat, 'for', this.indexCount, 'indices');
    }

    updateUniforms(viewProjectionMatrix, modelMatrix, normalMatrix, cameraPosition) {
        if (!this.isInitialized) return;

        const uniformData = new Float32Array(80); // Adjusted size for proper alignment
        
        // View-projection matrix (16 floats) - offset 0
        uniformData.set(viewProjectionMatrix, 0);
        
        // Model matrix (16 floats) - offset 16
        uniformData.set(modelMatrix, 16);
        
        // Normal matrix (3x3, stored as 3x4 for alignment) - offset 32
        // First column
        uniformData[32] = normalMatrix[0];
        uniformData[33] = normalMatrix[1];
        uniformData[34] = normalMatrix[2];
        uniformData[35] = 0.0; // padding
        
        // Second column
        uniformData[36] = normalMatrix[3];
        uniformData[37] = normalMatrix[4];
        uniformData[38] = normalMatrix[5];
        uniformData[39] = 0.0; // padding
        
        // Third column
        uniformData[40] = normalMatrix[6];
        uniformData[41] = normalMatrix[7];
        uniformData[42] = normalMatrix[8];
        uniformData[43] = 0.0; // padding
        
        // Camera position (3 floats) - offset 44
        uniformData.set(cameraPosition, 44);

        this.device.queue.writeBuffer(this.uniformBuffer, 0, uniformData);
    }

    updateTextures(diffuseTexture, normalTexture, specularTexture, emissionTexture) {
        if (!this.isInitialized) return;

        // Update texture bind group with new textures
        const sampler = this.device.createSampler({
            magFilter: 'linear',
            minFilter: 'linear',
            addressModeU: 'repeat',
            addressModeV: 'repeat',
        });

        this.textureBindGroup = this.device.createBindGroup({
            label: 'Texture Bind Group',
            layout: this.renderPipeline.getBindGroupLayout(0),
            entries: [
                {
                    binding: 0,
                    resource: {
                        buffer: this.uniformBuffer,
                    },
                },
                {
                    binding: 1,
                    resource: (diffuseTexture || this.defaultDiffuseTexture).createView(),
                },
                {
                    binding: 2,
                    resource: (normalTexture || this.defaultNormalTexture).createView(),
                },
                {
                    binding: 3,
                    resource: (specularTexture || this.defaultSpecularTexture).createView(),
                },
                {
                    binding: 4,
                    resource: (emissionTexture || this.defaultEmissionTexture).createView(),
                },
                {
                    binding: 5,
                    resource: sampler,
                },
            ],
        });
    }

    render() {
        if (!this.isInitialized || !this.vertexBuffer || !this.indexBuffer) return;

        const commandEncoder = this.device.createCommandEncoder();
        const textureView = this.context.getCurrentTexture().createView();

        const renderPassDescriptor = {
            colorAttachments: [
                {
                    view: textureView,
                    clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
                    loadOp: 'clear',
                    storeOp: 'store',
                },
            ],
            depthStencilAttachment: {
                view: this.depthTexture.createView(),
                depthClearValue: 1.0,
                depthLoadOp: 'clear',
                depthStoreOp: 'store',
            },
        };

        const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor);
        passEncoder.setPipeline(this.renderPipeline);
        passEncoder.setBindGroup(0, this.textureBindGroup);
        passEncoder.setVertexBuffer(0, this.vertexBuffer);
        passEncoder.setIndexBuffer(this.indexBuffer, this.indexFormat || 'uint16');
        passEncoder.drawIndexed(this.indexCount);
        passEncoder.end();

        this.device.queue.submit([commandEncoder.finish()]);
    }

    createTextureFromImageData(imageData) {
        const texture = this.device.createTexture({
            size: [imageData.width, imageData.height],
            format: 'rgba8unorm',
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
        });

        this.device.queue.writeTexture(
            { texture },
            imageData.data,
            { bytesPerRow: imageData.width * 4 },
            { width: imageData.width, height: imageData.height }
        );

        return texture;
    }

    resize(width, height) {
        this.canvas.width = width;
        this.canvas.height = height;
        
        if (this.depthTexture) {
            this.depthTexture.destroy();
        }
        
        this.depthTexture = this.device.createTexture({
            size: [width, height],
            format: 'depth24plus',
            usage: GPUTextureUsage.RENDER_ATTACHMENT,
        });
    }
}
