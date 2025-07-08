/**
 * Matrix utilities for WebGPU rendering
 */

export class Matrix4 {
    constructor(elements) {
        this.elements = elements || new Float32Array(16);
        if (!elements) {
            this.identity();
        }
    }

    identity() {
        const e = this.elements;
        e[0] = 1; e[4] = 0; e[8] = 0; e[12] = 0;
        e[1] = 0; e[5] = 1; e[9] = 0; e[13] = 0;
        e[2] = 0; e[6] = 0; e[10] = 1; e[14] = 0;
        e[3] = 0; e[7] = 0; e[11] = 0; e[15] = 1;
        return this;
    }

    perspective(fov, aspect, near, far) {
        const f = 1.0 / Math.tan(fov * 0.5);
        const rangeInv = 1.0 / (near - far);
        
        const e = this.elements;
        e[0] = f / aspect; e[4] = 0; e[8] = 0; e[12] = 0;
        e[1] = 0; e[5] = f; e[9] = 0; e[13] = 0;
        e[2] = 0; e[6] = 0; e[10] = (near + far) * rangeInv; e[14] = near * far * rangeInv * 2;
        e[3] = 0; e[7] = 0; e[11] = -1; e[15] = 0;
        
        return this;
    }

    lookAt(eye, target, up) {
        const z = this.normalize([
            eye[0] - target[0],
            eye[1] - target[1],
            eye[2] - target[2]
        ]);
        
        const x = this.normalize(this.cross(up, z));
        const y = this.cross(z, x);
        
        const e = this.elements;
        e[0] = x[0]; e[4] = x[1]; e[8] = x[2]; e[12] = -this.dot(x, eye);
        e[1] = y[0]; e[5] = y[1]; e[9] = y[2]; e[13] = -this.dot(y, eye);
        e[2] = z[0]; e[6] = z[1]; e[10] = z[2]; e[14] = -this.dot(z, eye);
        e[3] = 0; e[7] = 0; e[11] = 0; e[15] = 1;
        
        return this;
    }

    multiply(other) {
        const a = this.elements;
        const b = other.elements;
        const result = new Float32Array(16);
        
        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                result[i * 4 + j] = 
                    a[i * 4 + 0] * b[0 * 4 + j] +
                    a[i * 4 + 1] * b[1 * 4 + j] +
                    a[i * 4 + 2] * b[2 * 4 + j] +
                    a[i * 4 + 3] * b[3 * 4 + j];
            }
        }
        
        this.elements = result;
        return this;
    }

    translate(x, y, z) {
        const translation = new Matrix4();
        const e = translation.elements;
        e[12] = x;
        e[13] = y;
        e[14] = z;
        
        return this.multiply(translation);
    }

    rotate(angleX, angleY, angleZ) {
        if (angleX !== 0) {
            const cos = Math.cos(angleX);
            const sin = Math.sin(angleX);
            const rx = new Matrix4();
            const e = rx.elements;
            e[5] = cos; e[6] = sin;
            e[9] = -sin; e[10] = cos;
            this.multiply(rx);
        }
        
        if (angleY !== 0) {
            const cos = Math.cos(angleY);
            const sin = Math.sin(angleY);
            const ry = new Matrix4();
            const e = ry.elements;
            e[0] = cos; e[2] = -sin;
            e[8] = sin; e[10] = cos;
            this.multiply(ry);
        }
        
        if (angleZ !== 0) {
            const cos = Math.cos(angleZ);
            const sin = Math.sin(angleZ);
            const rz = new Matrix4();
            const e = rz.elements;
            e[0] = cos; e[1] = sin;
            e[4] = -sin; e[5] = cos;
            this.multiply(rz);
        }
        
        return this;
    }

    scale(x, y, z) {
        const scale = new Matrix4();
        const e = scale.elements;
        e[0] = x;
        e[5] = y || x;
        e[10] = z || x;
        
        return this.multiply(scale);
    }

    invert() {
        const e = this.elements;
        const inv = new Float32Array(16);
        
        inv[0] = e[5] * e[10] * e[15] -
                e[5] * e[11] * e[14] -
                e[9] * e[6] * e[15] +
                e[9] * e[7] * e[14] +
                e[13] * e[6] * e[11] -
                e[13] * e[7] * e[10];

        inv[4] = -e[4] * e[10] * e[15] +
                e[4] * e[11] * e[14] +
                e[8] * e[6] * e[15] -
                e[8] * e[7] * e[14] -
                e[12] * e[6] * e[11] +
                e[12] * e[7] * e[10];

        inv[8] = e[4] * e[9] * e[15] -
                e[4] * e[11] * e[13] -
                e[8] * e[5] * e[15] +
                e[8] * e[7] * e[13] +
                e[12] * e[5] * e[11] -
                e[12] * e[7] * e[9];

        inv[12] = -e[4] * e[9] * e[14] +
                e[4] * e[10] * e[13] +
                e[8] * e[5] * e[14] -
                e[8] * e[6] * e[13] -
                e[12] * e[5] * e[10] +
                e[12] * e[6] * e[9];

        inv[1] = -e[1] * e[10] * e[15] +
                e[1] * e[11] * e[14] +
                e[9] * e[2] * e[15] -
                e[9] * e[3] * e[14] -
                e[13] * e[2] * e[11] +
                e[13] * e[3] * e[10];

        inv[5] = e[0] * e[10] * e[15] -
                e[0] * e[11] * e[14] -
                e[8] * e[2] * e[15] +
                e[8] * e[3] * e[14] +
                e[12] * e[2] * e[11] -
                e[12] * e[3] * e[10];

        inv[9] = -e[0] * e[9] * e[15] +
                e[0] * e[11] * e[13] +
                e[8] * e[1] * e[15] -
                e[8] * e[3] * e[13] -
                e[12] * e[1] * e[11] +
                e[12] * e[3] * e[9];

        inv[13] = e[0] * e[9] * e[14] -
                e[0] * e[10] * e[13] -
                e[8] * e[1] * e[14] +
                e[8] * e[2] * e[13] +
                e[12] * e[1] * e[10] -
                e[12] * e[2] * e[9];

        inv[2] = e[1] * e[6] * e[15] -
                e[1] * e[7] * e[14] -
                e[5] * e[2] * e[15] +
                e[5] * e[3] * e[14] +
                e[13] * e[2] * e[7] -
                e[13] * e[3] * e[6];

        inv[6] = -e[0] * e[6] * e[15] +
                e[0] * e[7] * e[14] +
                e[4] * e[2] * e[15] -
                e[4] * e[3] * e[14] -
                e[12] * e[2] * e[7] +
                e[12] * e[3] * e[6];

        inv[10] = e[0] * e[5] * e[15] -
                e[0] * e[7] * e[13] -
                e[4] * e[1] * e[15] +
                e[4] * e[3] * e[13] +
                e[12] * e[1] * e[7] -
                e[12] * e[3] * e[5];

        inv[14] = -e[0] * e[5] * e[14] +
                e[0] * e[6] * e[13] +
                e[4] * e[1] * e[14] -
                e[4] * e[2] * e[13] -
                e[12] * e[1] * e[6] +
                e[12] * e[2] * e[5];

        inv[3] = -e[1] * e[6] * e[11] +
                e[1] * e[7] * e[10] +
                e[5] * e[2] * e[11] -
                e[5] * e[3] * e[10] -
                e[9] * e[2] * e[7] +
                e[9] * e[3] * e[6];

        inv[7] = e[0] * e[6] * e[11] -
                e[0] * e[7] * e[10] -
                e[4] * e[2] * e[11] +
                e[4] * e[3] * e[10] +
                e[8] * e[2] * e[7] -
                e[8] * e[3] * e[6];

        inv[11] = -e[0] * e[5] * e[11] +
                e[0] * e[7] * e[9] +
                e[4] * e[1] * e[11] -
                e[4] * e[3] * e[9] -
                e[8] * e[1] * e[7] +
                e[8] * e[3] * e[5];

        inv[15] = e[0] * e[5] * e[10] -
                e[0] * e[6] * e[9] -
                e[4] * e[1] * e[10] +
                e[4] * e[2] * e[9] +
                e[8] * e[1] * e[6] -
                e[8] * e[2] * e[5];

        let det = e[0] * inv[0] + e[1] * inv[4] + e[2] * inv[8] + e[3] * inv[12];

        if (det === 0) {
            return this; // Cannot invert
        }

        det = 1.0 / det;

        for (let i = 0; i < 16; i++) {
            this.elements[i] = inv[i] * det;
        }

        return this;
    }

    transpose() {
        const e = this.elements;
        const temp = new Float32Array(16);
        
        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                temp[i * 4 + j] = e[j * 4 + i];
            }
        }
        
        this.elements = temp;
        return this;
    }

    // Helper methods
    cross(a, b) {
        return [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0]
        ];
    }

    dot(a, b) {
        return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    }

    normalize(v) {
        const length = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
        if (length === 0) return [0, 0, 0];
        return [v[0] / length, v[1] / length, v[2] / length];
    }

    clone() {
        return new Matrix4(new Float32Array(this.elements));
    }
}

export class Matrix3 {
    constructor(elements) {
        this.elements = elements || new Float32Array(9);
        if (!elements) {
            this.identity();
        }
    }

    identity() {
        const e = this.elements;
        e[0] = 1; e[3] = 0; e[6] = 0;
        e[1] = 0; e[4] = 1; e[7] = 0;
        e[2] = 0; e[5] = 0; e[8] = 1;
        return this;
    }

    fromMatrix4(m4) {
        const e4 = m4.elements;
        const e3 = this.elements;
        
        e3[0] = e4[0]; e3[3] = e4[4]; e3[6] = e4[8];
        e3[1] = e4[1]; e3[4] = e4[5]; e3[7] = e4[9];
        e3[2] = e4[2]; e3[5] = e4[6]; e3[8] = e4[10];
        
        return this;
    }

    invert() {
        const e = this.elements;
        const inv = new Float32Array(9);
        
        inv[0] = e[4] * e[8] - e[5] * e[7];
        inv[1] = e[2] * e[7] - e[1] * e[8];
        inv[2] = e[1] * e[5] - e[2] * e[4];
        inv[3] = e[5] * e[6] - e[3] * e[8];
        inv[4] = e[0] * e[8] - e[2] * e[6];
        inv[5] = e[2] * e[3] - e[0] * e[5];
        inv[6] = e[3] * e[7] - e[4] * e[6];
        inv[7] = e[1] * e[6] - e[0] * e[7];
        inv[8] = e[0] * e[4] - e[1] * e[3];
        
        let det = e[0] * inv[0] + e[1] * inv[3] + e[2] * inv[6];
        
        if (det === 0) {
            return this; // Cannot invert
        }
        
        det = 1.0 / det;
        
        for (let i = 0; i < 9; i++) {
            this.elements[i] = inv[i] * det;
        }
        
        return this;
    }

    transpose() {
        const e = this.elements;
        const temp = new Float32Array(9);
        
        temp[0] = e[0]; temp[3] = e[1]; temp[6] = e[2];
        temp[1] = e[3]; temp[4] = e[4]; temp[7] = e[5];
        temp[2] = e[6]; temp[5] = e[7]; temp[8] = e[8];
        
        this.elements = temp;
        return this;
    }

    clone() {
        return new Matrix3(new Float32Array(this.elements));
    }
}
