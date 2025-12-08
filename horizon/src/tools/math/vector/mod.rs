use std::ops::{Add, Sub, Mul, Div};
use std::arch::x86_64::{self as simd};

//////////////////////
// AMBIGUOUS VECTOR //
//////////////////////

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Vector
{
    Length4(Vec4),
    Length3(Vec3),
    Length2(Vec2),
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        match self {
            Vector::Length2(vec2) => {
                (vec2 + Vec2::from(&other)).as_length()
            }
            Vector::Length3(vec3) => {
                (vec3 + Vec3::from(&other)).as_length()
            }
            Vector::Length4(vec4) => {
                (vec4 + Vec4::from(&other)).as_length()
            }
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        match self {
            Vector::Length2(vec2) => {
                (vec2 - Vec2::from(&other)).as_length()
            }
            Vector::Length3(vec3) => {
                (vec3 - Vec3::from(&other)).as_length()
            }
            Vector::Length4(vec4) => {
                (vec4 - Vec4::from(&other)).as_length()
            }
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        match self {
            Vector::Length2(vec2) => {
                (vec2 * Vec2::from(&other)).as_length()
            }
            Vector::Length3(vec3) => {
                (vec3 * Vec3::from(&other)).as_length()
            }
            Vector::Length4(vec4) => {
                (vec4 * Vec4::from(&other)).as_length()
            }
        }
    }
}

impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        match self {
            Vector::Length2(vec2) => {
                (vec2 / Vec2::from(&other)).as_length()
            }
            Vector::Length3(vec3) => {
                (vec3 / Vec3::from(&other)).as_length()
            }
            Vector::Length4(vec4) => {
                (vec4 / Vec4::from(&other)).as_length()
            }
        }
    }
}

impl Vectorable for Vector {
    fn dot(&self, other: &Self) -> f32 {
        match self {
            Vector::Length2(vec2) => {
                vec2.dot(&Vec2::from(other))
            }
            Vector::Length3(vec3) => {
                vec3.dot(&Vec3::from(other))
            }
            Vector::Length4(vec4) => {
                vec4.dot(&Vec4::from(other))
            }
        }
    }

    fn length(&self) -> f32 {
        match self {
            Vector::Length2(vec2) => {
                vec2.length()
            }
            Vector::Length3(vec3) => {
                vec3.length()
            }
            Vector::Length4(vec4) => {
                vec4.length()
            }
        }
    }

    fn as_length(self) -> Vector {
        self
    }
}



#[allow(unused)]
pub trait Vectorable<T = Self> {
    fn dot(&self, other: &T) -> f32;
    fn length(&self) -> f32;
    fn as_length(self) -> Vector;
}

//////////////
// VECTOR 4 //
/////////////
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub data: [f32; 4]
}

/* constructor */
#[allow(unused)]
impl  Vec4 {
    pub fn new (vals: [f32; 4]) -> Self
    {
        Self { 
            data: vals
        }
    }

    pub fn from (vec: &Vector) -> Self
    {
        match vec {
            Vector::Length2(vec2) => {
                Self {
                    data: [vec2.data[0].clone(), vec2.data[1].clone(), 0.0, 0.0]
                }
            }
            Vector::Length3(vec3) => {
                Self {
                    data: [vec3.data[0].clone(), vec3.data[1].clone(), vec3.data[2].clone(), 0.0]
                }
            }
            Vector::Length4(vec4) => {
                Self {
                    data: vec4.data.clone(),
                }
            }
        }
    }
}

/* Operators ----- */
impl Add for Vec4
{
    type Output = Vec4;
    fn add(self, other: Vec4) -> Vec4 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_add_ps(a_reg, b_reg);
            let mut res: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec4 { 
                data: res
            }
        }
    }
}

impl Sub for Vec4
{
    type Output = Vec4;
    fn sub(self, other: Vec4) -> Vec4 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_sub_ps(a_reg, b_reg);
            let mut res: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec4 { 
                data: res
            }
        }
    }
}

impl Mul for Vec4
{
    type Output = Vec4;
    fn mul(self, other: Vec4) -> Vec4 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            let mut res: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec4 { 
                data: res
            }
        }
    }
}

impl Div for Vec4
{
    type Output = Vec4;
    fn div(self, other: Vec4) -> Vec4 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_div_ps(a_reg, b_reg);
            let mut res: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec4 { 
                data: res
            }
        }
    }
}
/* ----- Operators */

// vector specific operations
impl Vectorable for Vec4
{
    fn dot(&self, other: &Vec4) -> f32 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::from_bits(res_bits as u32)
        }
    }

    fn length(&self) -> f32 {
        unsafe {
            let reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(reg, reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::sqrt(f32::from_bits(res_bits as u32))
        }
    }

    fn as_length(self) -> Vector {
        Vector::Length4(self)
    }
}


//////////////
// VECTOR 3 //
//////////////
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 { 
    pub data: [f32; 3 ] 
}

/* constructor */
#[allow(unused)]
impl Vec3
{
    pub fn new (vals: [f32; 3]) -> Self
    {
        Self {
            data: vals
        }
    }

    pub fn from(vec: &Vector) -> Self {
        match vec {
            Vector::Length2(vec2) => {
                Self {
                    data: [vec2.data[0].clone(), vec2.data[1].clone(), 0.0]
                }
            }
            Vector::Length3(vec3) => {
                Self {
                    data: vec3.data.clone()
                }
            }
            Vector::Length4(vec4) => {
                Self {
                    data: [vec4.data[0].clone(), vec4.data[1].clone(), vec4.data[2].clone()]
                }
            }
        }
    }
}

/* Operators -----  */
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_add_ps(a_reg, b_reg);
            let mut res: [f32; 3] = [0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec3 { 
                data: res,
            }
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_sub_ps(a_reg, b_reg);
            let mut res: [f32; 3] = [0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec3 { 
                data: res,
            }
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            let mut res: [f32; 3] = [0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec3 { 
                data: res,
            }
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_div_ps(a_reg, b_reg);
            let mut res: [f32; 3] = [0.0, 0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec3 { 
                data: res,
            }
        }
    }
}
/* ----- Operators */

// vector specific operations
impl Vectorable for Vec3
{
    fn dot(&self, other: &Self) -> f32 {
        unsafe {
            let a_aligned = self.data.align_to::<[f32; 4]>();
            let b_aligned = other.data.align_to::<[f32; 4]>();
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(a_aligned.0.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(b_aligned.0.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            let mask = simd::_mm_load_ps([1.0, 1.0, 1.0, 0.0].as_ptr());
            res_reg = simd::_mm_mul_ps(res_reg, mask);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::from_bits(res_bits as u32)
        }
    }

    fn length(&self) -> f32 {
        unsafe
        {
            let aligned = self.data.align_to::<[f32; 4]>();
            let reg: simd::__m128 = simd::_mm_loadu_ps(aligned.0.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(reg, reg);
            let mask: simd::__m128 = simd::_mm_loadu_ps([1.0, 1.0, 1.0, 0.0].as_ptr());
            res_reg = simd::_mm_mul_ps(res_reg, mask);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::sqrt(f32::from_bits(res_bits as u32))
        }
    }

    fn as_length(self) -> Vector {
        Vector::Length3(self)
    }
}

//////////////
// VECTOR 2 //
//////////////
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub data: [f32; 2]
}

/* constructor */
#[allow(unused)]
impl Vec2 {
    pub fn new (vals: [f32; 2]) -> Self
    {
        Self {
            data: vals,
        }
    }

    pub fn from (vec: &Vector) -> Self
    {
        match vec {
            Vector::Length2(vec2) => {
                Self {
                    data: vec2.data.clone()
                }
            }
            Vector::Length3(vec3) => {
                Self {
                    data: [vec3.data[0].clone(), vec3.data[1].clone()]
                }
            }
            Vector::Length4(vec4) => {
                Self {
                    data: [vec4.data[0].clone(), vec4.data[1].clone()]
                }
            }
        }
    }
}

/* operators ----- */
impl Add for Vec2
{
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_add_ps(a_reg, b_reg);
            let mut res: [f32; 2] = [0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec2 { 
                data: res
            }
        }
    }
}

impl Sub for Vec2
{
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_sub_ps(a_reg, b_reg);
            let mut res: [f32; 2] = [0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec2 { 
                data: res
            }
        }
    }
}

impl Mul for Vec2
{
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            let mut res: [f32; 2] = [0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec2 { 
                data: res
            }
        }
    }
}

impl Div for Vec2
{
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        unsafe {
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(self.data.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(other.data.as_ptr());
            let res_reg: simd::__m128 = simd::_mm_div_ps(a_reg, b_reg);
            let mut res: [f32; 2] = [0.0, 0.0];
            simd::_mm_storeu_ps(res.as_mut_ptr(), res_reg);
            Vec2 { 
                data: res
            }
        }
    }
}
/* ----- operators */

// vector specific operations
impl Vectorable for Vec2
{
    fn dot(&self, other: &Vec2) -> f32 {
        unsafe {
            let a_aligned = self.data.align_to::<[f32; 4]>();
            let b_aligned = other.data.align_to::<[f32; 4]>();
            let a_reg: simd::__m128 = simd::_mm_loadu_ps(a_aligned.0.as_ptr());
            let b_reg: simd::__m128 = simd::_mm_loadu_ps(b_aligned.0.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(a_reg, b_reg);
            let mask: simd::__m128 = simd::_mm_load_ps([1.0, 1.0, 0.0, 0.0].as_ptr());
            res_reg = simd::_mm_mul_ps(res_reg, mask);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::from_bits(res_bits as u32)
        }
    }

    fn length(&self) -> f32 {
        unsafe {
            let aligned = self.data.align_to::<[f32; 4]>();
            let reg: simd::__m128 = simd::_mm_loadu_ps(aligned.0.as_ptr());
            let mut res_reg: simd::__m128 = simd::_mm_mul_ps(reg, reg);
            let mask: simd::__m128 = simd::_mm_load_ps([1.0, 1.0, 0.0, 0.0].as_ptr());
            res_reg = simd::_mm_mul_ps(res_reg, mask);
            res_reg = simd::_mm_hadd_ps(res_reg, res_reg);
            let res_bits = simd::_mm_extract_ps::<1>(res_reg);
            f32::sqrt(f32::from_bits(res_bits as u32))
        }
    }

    fn as_length(self) -> Vector {
        Vector::Length2(self)
    }
}