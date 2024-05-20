//! Trait defining common DAC operations

/// This trait defines some common operations for a DAC
/// 
/// Where
///  - `D` = The data type used by the DAC. This is usually u8 or u16 but it could be a special type to hold non-byte bounded data.
///  - `E` = The error type to be returned in the results.
/// 
pub trait Dac<D, E> {
    type CalData;

    /// Set the output value for the DAC to `value`.
    /// 
    fn set_value(&mut self, value: D) -> Result<(), E>;

    /// Perform a manual or automatic calibration using `cal`
    /// 
    /// A struct implementing calibrate must define `type CalData`.
    /// If calibration isn't supported, the default implementation is adequate. Implementing struct should define `CalData=()`
    /// 
    /// ## Manual calibration
    ///  - Create a type `CalData` to store needed calibration data.
    ///  - Set the calibration data and return `cal`
    /// 
    /// ## Automatic calibration
    ///  - Create a type `CalData` to store the results of calibration. This will usually implement `Default`
    ///  - Perform an auto calibration sequence.
    ///  - Return the calculated calibration data if available.
    /// 
    fn calibrate<'a>(&mut self, cal: &'a Self::CalData) -> Result<&'a Self::CalData, E> {
        Ok(cal)
    }
}
