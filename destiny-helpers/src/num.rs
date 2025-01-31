/// 判断是否为0
/// <br> 如果[`val`]的绝对值小于[`1e-8`],则认为[`val`]为0
pub fn is_zero(val: f64) -> bool {
    val.abs() < 1e-8
}

pub trait F64NumSupport {
    /// 截断数值
    /// <br> [`decimals`]: 表示小数点后保留的位数.
    /// <br> [`round_up`]: 表示是否四舍五入.
    fn truncate(&self, decimals: u32, round_up: bool) -> f64;
    /// 将数值转换为安全的数值
    /// <br> 按固定小数位8位进行截断
    fn to_safe(&self) -> f64;
    /// 判断是否为0
    /// <br> 如果[`val`]的绝对值小于[`1e-8`],则认为[`val`]为0
    fn is_zero(&self) -> bool;
}

impl F64NumSupport for f64 {
    fn truncate(&self, decimals: u32, round_up: bool) -> f64 {
        let pow10 = 10i64.pow(decimals) as f64;
        let mut val = (self * pow10) as u64;
        if round_up {
            val += 1;
        }
        val as f64 / pow10
    }

    fn to_safe(&self) -> f64 {
        self.truncate(8, false)
    }

    fn is_zero(&self) -> bool {
        self.abs() < 1e-8
    }
}
