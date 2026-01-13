/// 2D 变换
/// 旋转、缩放、平移, 仿射变换
/// x.x y.x origin.x
/// x.y y.y origin.y
/// 0.0 0.0 1.0
pub struct Transform2D {
    pub origin: glam::Vec2,
    pub x: glam::Vec2,
    pub y: glam::Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform2D {

    pub fn new() -> Transform2D {
        Transform2D {
            origin: glam::Vec2::ZERO,
            x: glam::Vec2::X,
            y: glam::Vec2::Y,
        }
    }

    pub fn to_mat3(&self) -> glam::Mat3 { 
        glam::Mat3::from_cols(
            self.x.extend(0.0),
            self.y.extend(0.0),
            self.origin.extend(1.0),
        )
    }

    pub fn from_transform2d(transform: &Transform2D) -> Transform2D {
        Transform2D {
            origin: transform.origin,
            x: transform.x,
            y: transform.y,
        }
    }

    pub fn from_rot_and_pos(rotation: f32, position: glam::Vec2) -> Transform2D { 
        let origin = position;
        let sin = rotation.sin();
        let cos = rotation.cos();
        let x = glam::Vec2::new(cos, sin);
        let y = glam::Vec2::new(-sin, cos);

        Transform2D { origin, x, y }
    }

    pub fn from_rot_scale_skew_and_pos(rotation: f32, scale: glam::Vec2, skew: f32, position: glam::Vec2) -> Transform2D { 
        let origin = position;
        let sin = rotation.sin();
        let cos = rotation.cos();
        let tan = skew.tan();

        let x = glam::Vec2::new(
            scale.x * cos,
            scale.x * sin,
        );

        let y = glam::Vec2::new(
            scale.y * (-sin) + tan * x.x , 
            scale.y * cos + tan * x.y
        );

        Transform2D { origin, x, y }
    }

    pub fn from_x_axis_y_axis_and_origin(x: glam::Vec2, y: glam::Vec2, origin: glam::Vec2) -> Transform2D { 
        Transform2D { origin, x, y }
    }

    /// 仿射变换的逆矩阵
    pub fn affine_inverse(&self) -> Transform2D { 
        let det = self.x.x * self.y.y - self.y.x * self.x.y;

        if det.abs() < f32::EPSILON { panic!("Transform is not invertible (determinant is zero)") }

        let inv_det = 1.0 / det;

        let inv_x = glam::Vec2::new( self.y.y * inv_det, -self.x.y * inv_det);
        let inv_y = glam::Vec2::new(-self.y.x * inv_det,  self.x.x * inv_det);

        let new_origin = -(inv_x * self.origin.x + inv_y * self.origin.y);

        Transform2D::from_x_axis_y_axis_and_origin(inv_x, inv_y, new_origin)
    }

    /// 返回由变换基矩阵转换后的向量副本。这忽略了原点（即不应用平移）
    pub fn basis_xform(&self, v: glam::Vec2) -> glam::Vec2 {
        self.x * v.x + self.y * v.y
    }

    /// 返回向量的副本，经过逆变换基底矩阵的变换（乘以）（参见反变换）。该方法忽略了原点。
    /// 注：该方法假设该变换的基是正交归一（参见正交归一化）。
    /// 如果基不是正交归一，则应使用（参见affine_inverse（））。
    /// transform.affine_inverse().basis_xform(vector)
    pub fn basis_xform_inv(&self, v: glam::Vec2) -> glam::Vec2 { 
        glam::Vec2::new(
            v.dot(self.x),
            v.dot(self.y),
        )
    }

    /// 返回变换的行列式
    pub fn determinant(&self) -> f32 { 
        self.x.x * self.y.y - self.y.x * self.x.y
    }

    /// 获取变换的平移向量
    pub fn get_origin(&self) -> glam::Vec2 { 
        self.origin
    }

    /// 获取变换的旋转角度
    pub fn get_rotation(&self) -> f32 { 
        self.x.y.atan2(self.x.x)
    }

    /// 获取变换的缩放向量
    pub fn get_scale(&self) -> glam::Vec2 {
        glam::Vec2::new(self.x.length(), self.y.length())
    }

    pub fn get_skew(&self) -> f32 {
        let sx = self.x.length();
        let sy = self.y.length();

        // Avoid division by zero
        if sx < f32::EPSILON || sy < f32::EPSILON {
            return 0.0;
        }

        // Remove scale: get unit axes
        let x_unit = self.x / sx;
        let y_unit = self.y / sy;

        // In a pure rotation (no skew), y_unit should be perpendicular to x_unit:
        //   ideal_y = Vec2::new(-x_unit.y, x_unit.x)
        //
        // The skew angle is the angle between actual y_unit and ideal_y
        let ideal_y = glam::Vec2::new(-x_unit.y, x_unit.x);

        // Compute angle from ideal_y to y_unit
        let dot = ideal_y.dot(y_unit);
        let cross = ideal_y.x * y_unit.y - ideal_y.y * y_unit.x;
        cross.atan2(dot)
    }

    /// 返回该变换与线性插值的结果，由给定的 。xform weight
    /// 应该介于和（包含）之间。
    /// 允许超出该范围的值，并可用于进行外推。
    /// weight0.01.0
    pub fn interpolate_with(&self, xfrom: &Transform2D, weight: f32) -> Transform2D { 
        // Helper: lerp angle correctly (handles wrap-around at ±π)
        fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
            let diff = (b - a).rem_euclid(std::f32::consts::TAU);
            let shortest_diff = if diff > std::f32::consts::PI {
                diff - std::f32::consts::TAU
            } else {
                diff
            };
            a + shortest_diff * t
        }

        // Decompose self
        let pos_a = self.origin;
        let rot_a = self.get_rotation();
        let scale_a = self.get_scale();

        // Decompose xfrom
        let pos_b = xfrom.origin;
        let rot_b = xfrom.get_rotation();
        let scale_b = xfrom.get_scale();

        // Interpolate components
        let origin = pos_a.lerp(pos_b, weight);
        let rotation = lerp_angle(rot_a, rot_b, weight);
        let scale = scale_a.lerp(scale_b, weight);

        // Recompose
        let cos = rotation.cos();
        let sin = rotation.sin();

        Transform2D {
            origin,
            x: glam::Vec2::new(cos, sin) * scale.x,
            y: glam::Vec2::new(-sin, cos) * scale.y,
        }
    }
}