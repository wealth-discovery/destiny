/// 判断是否为0
/// <br> 如果[`val`]的绝对值小于[`1e-8`],则认为[`val`]为0
pub fn is_zero(val: f64) -> bool {
    val.abs() < 1e-8
}

pub trait F64NumSupport {
    /// 将数值转换为安全的数值
    /// <br> 按固定小数位8位进行截断
    fn to_safe(&self) -> f64;
    /// 判断是否为0
    /// <br> 如果[`val`]的绝对值小于[`1e-8`],则认为[`val`]为0
    fn is_zero(&self) -> bool;
}

impl F64NumSupport for f64 {
    fn to_safe(&self) -> f64 {
        (self * 10e8).round() / 10e8
    }

    fn is_zero(&self) -> bool {
        self.to_safe() == 0.
    }
}
