use std::ffi::*;
use serde::Deserialize;
use std::sync::{LazyLock, Mutex};

mod extern_functions;
use extern_functions::*;

pub mod error_codes;
pub use error_codes::*;

mod string_utils;
use string_utils::*;

type LicenseCallback = dyn Fn(LexActivatorCode) + Send + 'static;

static CALLBACK_FUNCTION: LazyLock<Mutex<Option<Box<LicenseCallback>>>> =
    LazyLock::new(|| Mutex::new(None));

extern "C" fn wrapper(code: i32) {
    let callback_status = LexActivatorCode::from_i32(code);
    let callback = CALLBACK_FUNCTION.lock().unwrap();
    if let Some(callback) = callback.as_ref() {
        callback(callback_status);
    }
}

/// Represents a license meter attribute.
#[derive(Debug)] 
pub struct LicenseMeterAttribute {
    /// The name of the meter attribute.
    pub name: String,
    /// The number of allowed uses for the meter attribute.
    pub allowed_uses: i64,
    /// The total number of uses recorded for the meter attribute.
    pub total_uses: u64,
    /// The gross number of uses for the meter attribute.
    pub gross_uses: u64
}

/// Represents a product version feature flag.
#[derive(Debug)] 
pub struct ProductVersionFeatureFlag {
    /// The name of the feature flag.
    pub name: String,
    /// Indicates whether the feature flag is enabled.
    pub enabled: bool,
    /// Additional data associated with the feature flag.
    pub data: String
}

/// Represents an activation mode.
#[derive(Debug)] 
pub struct ActivationMode {
    /// The initial activation mode.
    pub initial_mode: String,
    /// The current activation mode.
    pub current_mode: String
}

/// Represents a metadata 
#[derive(Debug, Deserialize, Default)]
pub struct Metadata {
    /// The key of the metadata.
    pub key: String,
    /// The value of the metadata.
    pub value: String,
}

/// Represents an organization address.
#[derive(Debug, Deserialize, Default)] 
pub struct OrganizationAddress {
    /// The first line of the address.
    #[serde(rename = "addressLine1")]
    pub address_line_1: String,
    /// The second line of the address.
    #[serde(rename = "addressLine2")]
    pub address_line_2: String,
    /// The city of the address.
    pub city: String,
    /// The state or region of the address.
    pub state: String,
    /// The country of the address.
    pub country: String,
    /// The postal code of the address.
    #[serde(rename = "postalCode")]
    pub postal_code: String
}

/// Represents a user license with information about various license parameters.
#[derive(Debug, Deserialize)] 
pub struct UserLicense {
    /// The allowed activations count of a license.
    #[serde(rename = "allowedActivations")]
    pub allowed_activations: i64,
    /// The allowed deactivations count of a license.
    #[serde(rename = "allowedDeactivations")]
    pub allowed_deactivations: i64,
    /// The license key.
    pub key: String,
    /// The license type.
    #[serde(rename = "type")]
    pub license_type: String,
    /// The license metadata.
    pub metadata: Vec<Metadata>
}

/// Represents a feature entitlement with details about its value.
#[derive(Debug, Deserialize)]
pub struct FeatureEntitlement {
    /// The name of the feature entitlement.
    #[serde(rename = "featureName")]
    pub feature_name: String,
    /// The value of the feature entitlement.
    #[serde(rename = "value")]
    pub value: String,
}

/// Represents various permission flags.
#[repr(u32)]
pub enum PermissionFlags {
    /// This flag indicates that the application does not require admin or root permissions to run
    LA_USER = 1,
    /// This flag indicates that the application must be run with admin or root permissions.
    LA_SYSTEM = 2,
    /// This flag is specifically designed for Windows and should be used for system-wide activations.
    LA_ALL_USERS = 3,
    /// This flag will store activation data in memory. Thus, requires re-activation on every start of 
    /// the application and should only be used in floating licenses.
    LA_IN_MEMORY = 4,
}

// --------------- Setter functions ------------------------

/// Embeds the Product.dat file in the application.
/// 
/// This function must be called on every start of your program before any other functions are called.
/// 
/// If this function fails to set the product data, none of the other functions will work.
/// 
/// # Arguments
/// 
/// * `product_data` - Content of the Product.dat file which you want to embed in your application.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the product data is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.


pub fn set_product_data(product_data: String) -> Result<(), LexActivatorError> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_product_data = to_utf16(product_data);
        status = unsafe { SetProductData(c_product_data.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_product_data = string_to_cstring(product_data)?;
        status = unsafe { SetProductData(c_product_data.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the product id of your application. 
/// 
/// This function must be called on every start of your program before any other functions are called, with the exception of SetProductData() function.
/// 
/// # Arguments
///
/// * `product_id` - A `string` value representing the unique product id of your application as mentioned
///                  on the product page in the dashboard.
///
/// * `permission_flags` - Depending on your application's requirements, choose one of 
///             the following values: LA_SYSTEM, LA_USER, LA_IN_MEMORY, LA_ALL_USERS.
///      
///     - `LA_USER`: This flag indicates that the application does not require
///        admin or root permissions to run.
///        
///     - `LA_SYSTEM`: This flag indicates that the application must be run with admin or root permissions.
/// 
///     - `LA_ALL_USERS`: This flag is specifically designed for Windows and should be used for system-wide activations.
/// 
///     - `LA_IN_MEMORY`: This flag will store activation data in memory. Thus, requires re-activation
///        on every start of the application and should only be used in floating licenses.
        
/// 
/// # Returns
///
/// Returns `Ok(())` if the data directory is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_product_id(product_id: String, permission_flags: PermissionFlags) -> Result<(), LexActivatorError> {
    let status: i32;
    let c_flags: c_uint = permission_flags as u32 as c_uint;
    #[cfg(windows)]
    {
        let c_product_id = to_utf16(product_id);
        status = unsafe { SetProductId(c_product_id.as_ptr(), c_flags) };
    }
    #[cfg(not(windows))]
    {
        
        let c_product_id = string_to_cstring(product_id)?;
        status = unsafe { SetProductId(c_product_id.as_ptr(), c_flags) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    } 
}

/// In case you want to change the default directory used by LexActivator to
/// store the activation data on Linux and macOS, this function can be used to
/// set a different directory.
/// 
/// # Arguments
/// 
/// * `data_dir` - A `string` value representing the absolute path of the directory
///               where LexActivator should store the activation data.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the data directory is set successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_data_directory(data_dir: String) -> Result<(), LexActivatorError> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_data_dir = to_utf16(data_dir);
        status = unsafe { SetDataDirectory(c_data_dir.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_data_dir = string_to_cstring(data_dir)?;
        status = unsafe { SetDataDirectory(c_data_dir.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Enables network logs.
///
/// This function should be used for network testing only in case of network errors. By default logging is disabled.
///
/// This function generates the lexactivator-logs.log file in the same directory where the application is running.
///
/// # Arguments
///
/// * `enable` - 0 or 1 to disable or enable logging.
///
/// Returns `Ok(())` if the debug mode is enabled successfully.

pub fn set_debug_mode(enable: u32) {
    let c_enable: c_uint = enable as c_uint;
    unsafe { SetDebugMode(c_enable) };
}

/// Enables or disables in-memory caching for LexActivator. 
/// 
/// This function is designed to control caching behavior to suit specific application requirements. 
/// 
/// Caching is enabled by default to enhance performance.
///
/// Disabling caching is recommended in environments where multiple processes access the same license on a 
///
/// single machine and require real-time updates to the license state.
///
/// # Arguments
///
/// * `mode` - False or True to disable or enable caching.
///
/// Returns `Ok(())` if mode is set successfully.

pub fn set_cache_mode(mode: bool) -> Result<(), LexActivatorError> {
    let c_mode: c_uint = if mode { 1 } else { 0 };  
    let status = unsafe { SetCacheMode(c_mode) };   
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// In case you don't want to use the LexActivator's advanced device fingerprinting algorithm, this function can be used to set a custom device fingerprint.
/// 
/// # Arguments
/// 
/// * `device_fingerprint` - A `string` value representing the custom device fingerprint of the user's device.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the custom device fingerprint is set successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_custom_device_fingerprint(device_fingerprint: String) -> Result<(), LexActivatorError> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_device_fingerprint = to_utf16(device_fingerprint);
        status = unsafe { SetCustomDeviceFingerprint(c_device_fingerprint.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_device_fingerprint = string_to_cstring(device_fingerprint)?;
        status = unsafe { SetCustomDeviceFingerprint(c_device_fingerprint.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the license key for activation.
///
/// # Arguments
///
/// * `license_key` - The license key string.
///
/// # Returns
///
/// Returns `Ok(())` if the license key is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.
 
pub fn set_license_key(license_key: String) -> Result<(), LexActivatorError> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_license_key = to_utf16(license_key);
        status = unsafe { SetLicenseKey(c_license_key.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_license_key = string_to_cstring(license_key)?;
        status = unsafe { SetLicenseKey(c_license_key.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the license user credentials for activation.
/// 
/// # Deprecated
/// This function is deprecated. Use [`authenticate_user`] instead.
/// 
/// # Arguments
///
/// * `email` - The email associated with the user.
/// * `password` - The password for the user.
///
/// # Returns
///
/// Returns `Ok(())` if the license user credentials are set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_license_user_credential(email: String, password: String) -> Result<(), LexActivatorError> {
    
    let status: i32;
    #[cfg(windows)]
    {
        let c_email = to_utf16(email);
        let c_password = to_utf16(password);
        status = unsafe { SetLicenseUserCredential(c_email.as_ptr(), c_password.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_email = string_to_cstring(email)?;
        let c_password = string_to_cstring(password)?;
        status = unsafe { SetLicenseUserCredential(c_email.as_ptr(), c_password.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the license closure callback.
///
/// Whenever the server sync occurs in a separate thread, and server returns the response,
/// license closure callback gets invoked with the following status codes:
/// LA_OK, LA_EXPIRED, LA_SUSPENDED, LA_E_REVOKED, LA_E_ACTIVATION_NOT_FOUND, LA_E_MACHINE_FINGERPRINT
/// LA_E_AUTHENTICATION_FAILED, LA_E_COUNTRY, LA_E_INET, LA_E_SERVER,LA_E_RATE_LIMIT, LA_E_IP,
/// LA_E_RELEASE_VERSION_NOT_ALLOWED, LA_E_RELEASE_VERSION_FORMAT
///
/// # Arguments
///
/// * `closure` - The closure callback to be set e.g. |code| { println!("{:?}", code) }
///
/// # Returns
///
/// Returns `Ok(())` if the license closure callback is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_license_callback<F>(closure: F) -> Result<(), LexActivatorError>
where
    F: Fn(LexActivatorCode) + Clone + Send + 'static,
{
    let mut callback_function = CALLBACK_FUNCTION.lock().unwrap();
    callback_function.replace(Box::new(closure));
    let status: i32 = unsafe { SetLicenseCallback(wrapper) };

    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Unset the current license closure callback.

pub fn unset_license_callback() {
    let mut callback_function = CALLBACK_FUNCTION.lock().unwrap();
    *callback_function = None;
}

/// Sets the activation lease duration.
///
/// # Arguments
///
/// * `lease_duration` - The lease duration in seconds. A value of -1 indicates unlimited lease duration.
///
/// # Returns
///
/// Returns `Ok(())` if the activation lease duration is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_activation_lease_duration(lease_duration: i64) -> Result<(), LexActivatorError> {
    let c_lease_duration: c_longlong = lease_duration as c_longlong;
    let status = unsafe { SetActivationLeaseDuration(c_lease_duration) };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the activation metadata.
///
/// # Arguments
///
/// * `key` - The key of the metadata.
/// * `value` - The value of the metadata.
///
/// # Returns
///
/// Returns `Ok(())` if the activation metadata is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_activation_metadata(key: String, value: String) -> Result<(), LexActivatorError>  {
    let status: i32;
    #[cfg(windows)]
    {
        let c_key = to_utf16(key);
        let c_value = to_utf16(value);
        status = unsafe { SetActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_key = string_to_cstring(key)?;
        let c_value = string_to_cstring(value)?;
        status = unsafe { SetActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the trial activation metadata.
///
/// # Arguments
///
/// * `key` - The key of the metadata.
/// * `value` - The value of the metadata.
///
/// # Returns
///
/// Returns `Ok(())` if the trial activation metadata is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_trial_activation_metadata(key: String, value: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_key = to_utf16(key);
        let c_value = to_utf16(value);
        status = unsafe { SetTrialActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_key = string_to_cstring(key)?;
        let c_value = string_to_cstring(value)?;
        status = unsafe { SetTrialActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the release version.
///
/// # Arguments
///
/// * `release_version` - The release version.
///
/// # Returns
///
/// Returns `Ok(())` if the release version is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_release_version(version: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_version = to_utf16(version);
        status = unsafe { SetReleaseVersion(c_version.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_version = string_to_cstring(version)?;
        status = unsafe { SetReleaseVersion(c_version.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the release published date.
///
/// # Arguments
///
/// * `release_published_date` - The release published date as a UNIX timestamp.
///
/// # Returns
///
/// Returns `Ok(())` if the release published date is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_release_published_date(release_published_date: u32) -> Result<(), LexActivatorError>{
    let c_release_published_date: c_uint = release_published_date as c_uint;
    let status = unsafe { SetReleasePublishedDate(c_release_published_date) };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the release platform.
///
/// # Arguments
///
/// * `platform` - The release platform.
///
/// # Returns
///
/// Returns `Ok(())` if the release platform is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_release_platform(platform: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_platform = to_utf16(platform);
        status = unsafe { SetReleasePlatform(c_platform.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_platform = string_to_cstring(platform)?;
        status = unsafe { SetReleasePlatform(c_platform.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the release channel e.g. stable, beta
///
/// # Arguments
///
/// * `release_channel` - The release channel.
///
/// # Returns
///
/// Returns `Ok(())` if the release channel is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_release_channel(channel: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_channel = to_utf16(channel);
        status = unsafe { SetReleaseChannel(c_channel.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_channel = string_to_cstring(channel)?;
        status = unsafe { SetReleaseChannel(c_channel.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the offline activation request meter attribute uses.
///
/// # Arguments
///
/// * `name` - The name of the meter attribute.
/// * `uses` - The number of uses.
///
/// # Returns
///
/// Returns `Ok(())` if the offline activation request meter attribute uses are set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_offline_activation_request_meter_attribute_uses(name: String, uses: i32) -> Result<(), LexActivatorError>{
    let status: i32;
    let c_uses: c_uint = uses as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { SetOfflineActivationRequestMeterAttributeUses(c_name.as_ptr(), c_uses) };
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name)?;
        status = unsafe { SetOfflineActivationRequestMeterAttributeUses(c_name.as_ptr(), c_uses) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the network proxy.
///
/// # Arguments
///
/// * `proxy` - The network proxy.
///
/// # Returns
///
/// Returns `Ok(())` if the network proxy is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_network_proxy(proxy: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_proxy = to_utf16(proxy);
        status = unsafe { SetNetworkProxy(c_proxy.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_proxy = string_to_cstring(proxy)?;
        status = unsafe { SetNetworkProxy(c_proxy.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the Cryptlex host.
///
/// # Arguments
///
/// * `host` - The Cryptlex host.
///
/// # Returns
///
/// Returns `Ok(())` if the Cryptlex host is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_cryptlex_host(host: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_host = to_utf16(host);
        status = unsafe { SetCryptlexHost(c_host.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_host = string_to_cstring(host)?;
        status = unsafe { SetCryptlexHost(c_host.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Sets the two-factor authentication code for the user authentication.
/// 
/// # Arguments
///
/// * `two_factor_authentication_code` - The 2FA code.
///
/// # Returns
///
/// Returns `Ok(())` if the two_factor_authentication_code is set successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn set_two_factor_authentication_code(two_factor_authentication_code: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_two_factor_authentication_code = to_utf16(two_factor_authentication_code);
        status = unsafe { SetTwoFactorAuthenticationCode(c_two_factor_authentication_code.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_two_factor_authentication_code = string_to_cstring(two_factor_authentication_code)?;
        status = unsafe { SetTwoFactorAuthenticationCode(c_two_factor_authentication_code.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

// ------------------- Getter Functions --------------------

pub fn get_product_metadata(key: String) -> Result<String, LexActivatorError> {
    
    let status: i32;
    const LENGTH: usize = 256;
    let product_metadata_value: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let utf16_ptr =  to_utf16(key);
        status = unsafe { GetProductMetadata(utf16_ptr.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_metadata_value = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let key_cstring: CString  = string_to_cstring(key)?;
        status = unsafe { GetProductMetadata(key_cstring.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_metadata_value = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(product_metadata_value)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the name of the product version.
///
/// # Deprecated
/// This function is deprecated. Use [`get_license_entitlement_set_name`] instead.
///
/// # Returns
///
/// Returns `Ok(String)` with the name of the product version if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_product_version_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256;
    let product_version_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_name = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(product_version_name)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the display name of the product version.
///
/// # Deprecated
/// This function is deprecated. Use [`get_license_entitlement_set_display_name`] instead.
///
/// # Returns
/// Returns `Ok(String)` with the display name of the product version if it is retrieved successfully.
/// If an error occurs, an `Err` containing the `LexActivatorError` is returned.

pub fn get_product_version_display_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let product_version_display_name: String;
    
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_display_name = utf16_to_string(&buffer);
    }

    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_display_name = c_char_to_string(&buffer);
    }

    if status == 0 {
        Ok(product_version_display_name)
    } else {
        Err(LexActivatorError::from(status))
    }
}

/// Retrieves the feature flag of a specific product version.
///
/// # Arguments
///
/// * `name` - The name of the feature flag.
///
/// # Returns
///
/// Returns `Ok(ProductVersionFeatureFlag)` with the feature flag information if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_product_version_feature_flag(name: String) -> Result<ProductVersionFeatureFlag, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let feature_name: String = name.clone();
    let data: String;
    let mut c_enabled: c_uint = 0;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionFeatureFlag(c_name.as_ptr(), &mut c_enabled, buffer.as_mut_ptr(), LENGTH as c_uint) };
        data = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name)?;
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetProductVersionFeatureFlag(c_name.as_ptr(), &mut c_enabled, buffer.as_mut_ptr(), LENGTH as c_uint) };
        data = c_char_to_string(&buffer);
    }
    let product_version_feature_flag = ProductVersionFeatureFlag {
        name: feature_name,
        enabled: u32_to_bool(c_enabled),
        data: data
    };
    if status == 0 {
        Ok(product_version_feature_flag)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the metadata associated with a license.
///
/// # Arguments
///
/// * `key` - The metadata key.
///
/// # Returns
///
/// Returns `Ok(String)` with the metadata value if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_metadata(key: String) -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let license_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key =  to_utf16(key);
        status = unsafe { GetLicenseMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key)?;
        status = unsafe { GetLicenseMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_metadata = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(license_metadata)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the meter attribute of a license.
///
/// # Arguments
///
/// * `name` - The name of the meter attribute.
///
/// # Returns
///
/// Returns `Ok(LicenseMeterAttribute)` with the meter attribute information if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_meterattribute(name: String) -> Result<LicenseMeterAttribute, LexActivatorError> {
    let status: i32;
    let meter_attribute_name: String = name.clone();
    let mut c_allowed_uses: c_longlong = 0;
    let mut c_total_uses: c_ulonglong = 0;
    let mut c_gross_uses: c_ulonglong = 0;
    #[cfg(windows)]
    {
        let c_name =  to_utf16(name);
        status = unsafe { GetLicenseMeterAttribute(c_name.as_ptr(), &mut c_allowed_uses, &mut c_total_uses, &mut c_gross_uses) };
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name)?;
        status = unsafe { GetLicenseMeterAttribute(c_name.as_ptr(), &mut c_allowed_uses, &mut c_total_uses, &mut c_gross_uses) };
    }
    let meter_attribute = LicenseMeterAttribute {
        name: meter_attribute_name,
        allowed_uses: c_allowed_uses,
        total_uses: c_total_uses,
        gross_uses: c_gross_uses,
    };
    if status == 0 {
        Ok(meter_attribute)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the license key.
///
/// # Returns
///
/// Returns `Ok(String)` with the license key if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_key() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let license_key: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseKey(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_key = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseKey(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_key = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(license_key)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the number of allowed activations for the license.
///
/// # Returns
///
/// Returns `Ok(i64)` with the number of allowed activations if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_allowed_activations() -> Result<i64, LexActivatorError> {
    let mut allowed_activations: c_longlong = 0;
    let status = unsafe { GetLicenseAllowedActivations(&mut allowed_activations) };
    if status == 0 {
        Ok(allowed_activations)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the total number of activations for the license.
///
/// # Returns
///
/// Returns `Ok(u32)` with the total number of activations if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_total_activations() -> Result<u32, LexActivatorError> {
    let mut total_activations: c_uint = 0;
    let status = unsafe { GetLicenseTotalActivations(&mut total_activations) };
    if status == 0 {
        Ok(total_activations)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the number of allowed deactivations for the license.
///
/// # Returns
///
/// Returns `Ok(i64)` with the number of allowed deactivations if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_allowed_deactivations() -> Result<i64, LexActivatorError> {
    let mut allowed_deactivations: c_longlong = 0;
    let status = unsafe { GetLicenseAllowedDeactivations(&mut allowed_deactivations) };
    if status == 0 {
        Ok(allowed_deactivations)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the total number of deactivations for the license.
///
/// # Returns
///
/// Returns `Ok(u32)` with the total number of deactivations if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_total_deactivations() -> Result<u32, LexActivatorError> {
    let mut total_deactivations: c_uint = 0;
    let status = unsafe { GetLicenseTotalDeactivations(&mut total_deactivations) };
    if status == 0 {
        Ok(total_deactivations)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the license creation date timestamp.
///
/// # Returns
///
/// Returns `Ok(u32)` with the license creation date timestamp if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_creation_date() -> Result<u32, LexActivatorError> {
    let mut creation_date:c_uint = 0;
    let status = unsafe { GetLicenseCreationDate(&mut creation_date) };
    if status == 0 {
        Ok(creation_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the license activation date timestamp.
///
/// # Returns
///
/// Returns `Ok(u32)` with the license activation date timestamp if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_activation_date() -> Result<u32, LexActivatorError> {
    let mut activation_date:c_uint = 0;
    let status = unsafe { GetLicenseActivationDate(&mut activation_date) };
    if status == 0 {
        Ok(activation_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the expiry date of the license.
///
/// # Returns
///
/// Returns `Ok(u32)` with the expiry date (in seconds since Unix epoch) if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_expiry_date() -> Result<u32, LexActivatorError> {
    let mut expiry_date: c_uint = 0;
    let status = unsafe { GetLicenseExpiryDate(&mut expiry_date) };
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the maintenance expiry date of the license.
///
/// # Returns
///
/// Returns `Ok(u32)` with the maintenance expiry date (in seconds since Unix epoch) if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_maintenance_expiry_date() -> Result<u32, LexActivatorError> {
    let mut expiry_date: c_uint = 0;
    let status = unsafe { GetLicenseMaintenanceExpiryDate(&mut expiry_date) };
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the maximum allowed release version for the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the maximum allowed release version if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_max_allowed_release_version() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let max_allowed_release_version: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseMaxAllowedReleaseVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        max_allowed_release_version = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseMaxAllowedReleaseVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        max_allowed_release_version = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(max_allowed_release_version)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the user's email associated with the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the user's email if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_user_email() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let user_email: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserEmail(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_email = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserEmail(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_email = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(user_email)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the user's name associated with the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the user's name if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_user_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let user_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_name = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(user_name)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the user's company associated with the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the user's company if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_user_company() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let user_company: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserCompany(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_company = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseUserCompany(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_company = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(user_company)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the metadata value associated with the license user.
///
/// # Arguments
///
/// * `key` - The key of the metadata value.
///
/// # Returns
///
/// Returns `Ok(String)` with the metadata value if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_user_metadata(key: String) -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let user_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { GetLicenseUserMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key)?;
        status = unsafe { GetLicenseUserMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_metadata = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(user_metadata)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the organization name associated with the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the organization name if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_organization_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let organization_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseOrganizationName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        organization_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseOrganizationName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        organization_name = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(organization_name)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the organization address associated with the license.
///
/// # Returns
///
/// Returns `Ok(OrganizationAddress)` with the organization address if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_organization_address() -> Result<OrganizationAddress, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let org_address_json: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseOrganizationAddressInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        org_address_json = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseOrganizationAddressInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        org_address_json = c_char_to_string(&buffer);
    }
    if status == 0 {
        if org_address_json.trim().is_empty() {
            Ok(OrganizationAddress::default())
        } else {
            let org_address: OrganizationAddress = serde_json::from_str(&org_address_json).expect("Failed to parse JSON");
            Ok(org_address)
        }
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the user licenses associated with the current user.
///
/// This function sends a network request to Cryptlex servers to get the licenses.
///
/// Make sure AuthenticateUser() function is called before calling this function.
///
/// # Returns
///
/// Returns `Ok(Vec<UserLicense>)` with the user licenses if retrieved successfully. If an error occurs, an `Err` containing the `LexActivatorError` is returned. 

pub fn get_user_licenses() -> Result<Vec<UserLicense>, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 1024;
    let user_licenses_json: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetUserLicensesInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_licenses_json = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetUserLicensesInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_licenses_json = c_char_to_string(&buffer);
    }
    if status == 0 {
        if user_licenses_json.is_empty() {
            Ok(Vec::new())
        } else {
            let user_licenses: Vec<UserLicense> = serde_json::from_str(&user_licenses_json).expect("Failed to parse JSON");
            Ok(user_licenses)
        }
        
    } else {
        Err(LexActivatorError::from(status))
    }
}

/// Retrieves the license entitlement set name.
///
/// # Returns
///
/// Returns `Ok(String)` with the entitlement set name of the license if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_entitlement_set_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256;
    let license_entitlement_set_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseEntitlementSetName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_entitlement_set_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseEntitlementSetName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_entitlement_set_name = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(license_entitlement_set_name)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the entitlement set display name.
///
/// # Returns
///
/// Returns `Ok(String)` with the entitlement set display name of the license if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_entitlement_set_display_name() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256;
    let license_entitlement_set_display_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseEntitlementSetDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_entitlement_set_display_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseEntitlementSetDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_entitlement_set_display_name = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(license_entitlement_set_display_name)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the feature entitlements.
///
/// # Returns
///
/// Returns `Ok(Vec<FeatureEntitlement>)` with the feature entitlements of the license if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.
pub fn get_feature_entitlements() -> Result<Vec<FeatureEntitlement>, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 4096;
    let feature_entitlements_json: String;

    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetFeatureEntitlementsInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        feature_entitlements_json = utf16_to_string(&buffer);
    }
    
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetFeatureEntitlementsInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        feature_entitlements_json = c_char_to_string(&buffer);
    }

    if status == 0 {
        if feature_entitlements_json.is_empty() {
            Ok(Vec::new())
        } else {
            let feature_entitlements: Vec<FeatureEntitlement> = serde_json::from_str(&feature_entitlements_json).expect("Failed to parse JSON");
            Ok(feature_entitlements)
        }
    } else {
        Err(LexActivatorError::from(status))
    }
}

/// Retrieves the feature entitlement.
///
/// # Arguments
///
/// * `feature_name` - A `string` value representing the name of the feature entitlement.
///
/// # Returns
///
/// Returns `Ok(FeatureEntitlement)` with the feature entitlement of the license if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.
pub fn get_feature_entitlement(feature_name: String) -> Result<FeatureEntitlement, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 1024;
    let feature_entitlement_json: String;

    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_name = to_utf16(name);
        status = unsafe { GetFeatureEntitlementInternal(c_name.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        feature_entitlement_json = utf16_to_string(&buffer);
    }
    
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(feature_name)?;
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetFeatureEntitlementInternal(c_name.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        feature_entitlement_json = c_char_to_string(&buffer);
    }

    if status == 0 {
        if feature_entitlement_json.is_empty() {
            Err(LexActivatorError::from(status))
        } else {
            let feature_entitlement: FeatureEntitlement = serde_json::from_str(&feature_entitlement_json).expect("Failed to parse JSON");
            Ok(feature_entitlement)
        }
    } else {
        Err(LexActivatorError::from(status))
    }
}

/// Retrieves the type of the license.
///
/// # Returns
///
/// Returns `Ok(String)` with the license type if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_license_type() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let license_type: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseType(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_type = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLicenseType(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_type = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(license_type)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the activation id.
///
/// # Returns
///
/// Returns `Ok(String)` with the activation id if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_activation_id() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256;
    let activation_id: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetActivationId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_id = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetActivationId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_id = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(activation_id)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the metadata value associated with the specified key for the activation.
///
/// # Arguments
///
/// * `key` - The key of the metadata value.
///
/// # Returns
///
/// Returns `Ok(String)` with the metadata value if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_activation_metadata(key: String) -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let activation_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { GetActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key)?;
        status = unsafe { GetActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_metadata = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(activation_metadata)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the initial and current activation mode.
///
/// # Returns
///
/// Returns `Ok(ActivationMode)` with the initial and current activation mode if they are retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_activation_mode() -> Result<ActivationMode, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let initial_activation_mode: String;
    let current_activation_mode: String;
    #[cfg(windows)]
    {
        let mut initial_mode_buffer: [u16; LENGTH] = [0; LENGTH];
        let mut current_mode_buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetActivationMode(initial_mode_buffer.as_mut_ptr(), LENGTH as c_uint, current_mode_buffer.as_mut_ptr(), LENGTH as c_uint) };
        initial_activation_mode = utf16_to_string(&initial_mode_buffer);
        current_activation_mode = utf16_to_string(&current_mode_buffer);
    }
    #[cfg(not(windows))]
    {
        let mut initial_mode_buffer: [c_char; LENGTH] = [0; LENGTH];
        let mut current_mode_buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetActivationMode(initial_mode_buffer.as_mut_ptr(), LENGTH as c_uint, current_mode_buffer.as_mut_ptr(), LENGTH as c_uint) };
        initial_activation_mode = c_char_to_string(&initial_mode_buffer);
        current_activation_mode = c_char_to_string(&current_mode_buffer);
    }
    let activation_mode = ActivationMode {
        initial_mode: initial_activation_mode,
        current_mode: current_activation_mode,
    };
    if status == 0 {
        Ok(activation_mode)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the number of uses of the specified metered attribute for the activation.
///
/// # Arguments
///
/// * `name` - The name of the metere attribute.
///
/// # Returns
///
/// Returns `Ok(u32)` with the number of uses of the metered attribute if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_activation_meter_attribute_uses(name: String) -> Result<u32, LexActivatorError> {
    let status: i32;
    let mut count: c_uint = 0;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { GetActivationMeterAttributeUses(c_name.as_ptr(), &mut count,) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name)?;
        status = unsafe { GetActivationMeterAttributeUses(c_name.as_ptr(), &mut count) };
    }
    if status == 0 {
        Ok(count)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the expiry date of the server sync grace period for the activation.
///
/// # Returns
///
/// Returns `Ok(u32)` with the expiry date of the server sync grace period if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_server_sync_grace_period_expiry_date() -> Result<u32, LexActivatorError> {
    let status: i32;
    let mut expiry_date: c_uint = 0;
    status = unsafe { GetServerSyncGracePeriodExpiryDate(&mut expiry_date) };
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the metadata value associated with the specified key for the trial activation.
///
/// # Arguments
///
/// * `key` - The key of the metadata value.
///
/// # Returns
///
/// Returns `Ok(String)` with the metadata value if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_trial_activation_metadata(key: String) -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let trial_activation_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { GetTrialActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_activation_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key)?;
        status = unsafe { GetTrialActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_activation_metadata = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(trial_activation_metadata)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the expiry date of the trial activation.
///
/// # Returns
///
/// Returns `Ok(u32)` with the expiry date of the trial activation if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_trial_expiry_date() -> Result<u32, LexActivatorError> {
    let status: i32;
    let mut trial_expiry_date: c_uint = 0;
    status = unsafe { GetTrialExpiryDate(&mut trial_expiry_date) };
    if status == 0 {
        Ok(trial_expiry_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the ID of the trial activation.
///
/// # Returns
///
/// Returns `Ok(String)` with the trial ID if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_trial_id() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let trial_id: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetTrialId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_id = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetTrialId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_id = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(trial_id)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the local expiry date of the trial activation.
///
/// # Returns
///
/// Returns `Ok(u32)` with the local expiry date of the trial activation if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_local_trial_expiry_date() -> Result<u32, LexActivatorError> {
    let status: i32;
    let mut trial_expiry_date: c_uint = 0;
    status = unsafe { GetLocalTrialExpiryDate(&mut trial_expiry_date) };
    if status == 0 {
        Ok(trial_expiry_date)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Retrieves the version of the LexActivator library.
///
/// # Returns
///
/// Returns `Ok(String)` with the library version if it is retrieved successfully, If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn get_library_version() -> Result<String, LexActivatorError> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let library_version: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { GetLibraryVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        library_version = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { GetLibraryVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        library_version = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(library_version)
    } else {
        return Err(LexActivatorError::from(status));
    }
}

// ------------------ Action Functions ------------------

/// Authenticates the user.
/// 
/// It sends the request to the Cryptlex servers to authenticate the user.
///
/// # Arguments
/// 
/// * `email` - user email address.
/// * `password` - user password.
///
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the authentication is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn authenticate_user(email: String, password: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_email = to_utf16(email);
        let c_password = to_utf16(password);
        status = unsafe { AuthenticateUser(c_email.as_ptr(), c_password.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_email = string_to_cstring(email)?;
        let c_password = string_to_cstring(password)?;
        status = unsafe { AuthenticateUser(c_email.as_ptr(), c_password.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Authenticates the user via OIDC Id token.
///
/// # Arguments
/// 
/// * `id_token` - The id token obtained from the OIDC provider.
/// 
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the authentication is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn authenticate_user_with_id_token(id_token: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_id_token = to_utf16(id_token);
        status = unsafe { AuthenticateUserWithIdToken(c_id_token.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_id_token = string_to_cstring(id_token)?;
        status = unsafe { AuthenticateUserWithIdToken(c_id_token.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Activates the license by contacting the Cryptlex servers. 
/// 
/// It validates the key and returns with encrypted and digitally signed token which it stores and uses to activate the application.
/// 
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the license activation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.
  
pub fn activate_license() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { ActivateLicense() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        20 => Ok(LexActivatorStatus::LA_EXPIRED),
        21 => Ok(LexActivatorStatus::LA_SUSPENDED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Activates your licenses using the offline activation response file.
/// 
/// # Arguments
/// 
/// * `file_path` - The path of the offline activation response file.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the license activation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn activate_license_offline(file_path: String) -> Result<LexActivatorStatus, LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { ActivateLicenseOffline(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path)?;
        status = unsafe { ActivateLicenseOffline(c_file_path.as_ptr()) };
    }
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        20 => Ok(LexActivatorStatus::LA_EXPIRED),
        21 => Ok(LexActivatorStatus::LA_SUSPENDED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Generates an offline activation request file. The request file contains necessary information to perform offline activation. 
///
/// # Arguments
///
/// * `file_path` - The path to save the offline activation request file.
///
/// # Returns
///
/// Returns `Ok(())` if the offline activation request file generation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn generate_offline_activation_request(file_path: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { GenerateOfflineActivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path)?;
        status = unsafe { GenerateOfflineActivationRequest(c_file_path.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Deactivates the license activation and frees up the corresponding activation slot by contacting the Cryptlex servers.
///
/// This function should be executed at the time of de-registration, ideally on a button click.
///
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the license deactivation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn deactivate_license() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { DeactivateLicense() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Generates the offline deactivation request needed for deactivation of the license in the dashboard and deactivates the license locally.
///
/// # Arguments
///
/// * `file_path` - The path to save the offline deactivation request file.
///
/// # Returns
///
/// Returns `Ok(())` if the offline deactivation request file generation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn generate_offline_deactivation_request(file_path: String) -> Result<LexActivatorStatus, LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { GenerateOfflineDeactivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path)?;
        status = unsafe { GenerateOfflineDeactivationRequest(c_file_path.as_ptr()) };
    }
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// It verifies whether your app is genuinely activated or not. The verification is done locally by verifying the cryptographic digital signature fetched at the time of activation.
///
/// After verifying locally, it schedules a server check in a separate thread. After the first server sync it periodically does further syncs at a frequency set for the license.
/// 
/// In case server sync fails due to network error, and it continues to fail for fixed number of days (grace period), the function returns LA_GRACE_PERIOD_OVER instead of LA_OK.
/// 
/// This function must be called on every start of your program to verify the activation of your app.
/// 
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the license is genuine. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn is_license_genuine() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { IsLicenseGenuine() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        20 => Ok(LexActivatorStatus::LA_EXPIRED),
        21 => Ok(LexActivatorStatus::LA_SUSPENDED),
        22 => Ok(LexActivatorStatus::LA_GRACE_PERIOD_OVER),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// It verifies whether your app is genuinely activated or not. The verification is done locally by verifying the cryptographic digital signature fetched at the time of activation.
/// 
/// This is just an auxiliary function which you may use in some specific cases, when you want to skip the server sync.
/// 
/// You may want to set grace period to 0 to ignore grace period.
/// 
/// # Returns
///
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the license is genuine. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn is_license_valid() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { IsLicenseValid() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        20 => Ok(LexActivatorStatus::LA_EXPIRED),
        21 => Ok(LexActivatorStatus::LA_SUSPENDED),
        22 => Ok(LexActivatorStatus::LA_GRACE_PERIOD_OVER),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Starts the verified trial in your application by contacting the Cryptlex servers.
/// 
/// This function should be executed when your application starts first time on the user's computer, ideally on a button click.
///
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the trial has started successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn activate_trial() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { ActivateTrial() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        25 => Ok(LexActivatorStatus::LA_TRIAL_EXPIRED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Activates the trial using the offline activation response file.
/// 
/// # Arguments
/// 
/// * `file_path` - path of the offline activation response file.
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the trial has started successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn activate_trial_offline(file_path: String) -> Result<LexActivatorStatus, LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { ActivateTrialOffline(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path)?;
        status = unsafe { ActivateTrialOffline(c_file_path.as_ptr()) };
    }
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        25 => Ok(LexActivatorStatus::LA_TRIAL_EXPIRED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Generates the offline trial activation request needed for generating offline trial activation response in the dashboard.
///
/// # Arguments
///
/// * `file_path` - path of the file path where the offline request has to be saved.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the offline trial activation request file generation is successful. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn generate_offline_trial_activation_request(file_path: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { GenerateOfflineTrialActivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path)?;
        status = unsafe { GenerateOfflineTrialActivationRequest(c_file_path.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// It verifies whether trial has started and is genuine or not. The verification is done locally by verifying the cryptographic digital signature fetched at the time of trial activation.
/// 
/// This function must be called on every start of your program during the trial period.
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if trial has started and is genuine. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn is_trial_genuine() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { IsTrialGenuine() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        25 => Ok(LexActivatorStatus::LA_TRIAL_EXPIRED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Starts the local(unverified) trial.
/// 
/// This function should be executed when your application starts first time on the user's computer.
/// 
/// The function is only meant for local(unverified) trials.
/// 
/// # Arguments
/// 
/// * `trial_length` - trial length in days
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the trial has started successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn activate_local_trial(trial_length: u32) -> Result<LexActivatorStatus, LexActivatorError> {
    let c_trial_length: c_uint = trial_length as c_uint;
    let status = unsafe { ActivateLocalTrial(c_trial_length) };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        26 => Ok(LexActivatorStatus::LA_LOCAL_TRIAL_EXPIRED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// It verifies whether trial has started and is genuine or not. The verification is done locally.
/// 
/// This function must be called on every start of your program during the trial period.
/// 
/// The function is only meant for local(unverified) trials.
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if trial is genuine. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn is_local_trial_genuine() -> Result<LexActivatorStatus, LexActivatorError> {
    let status = unsafe { IsLocalTrialGenuine() };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        26 => Ok(LexActivatorStatus::LA_LOCAL_TRIAL_EXPIRED),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Extends the local trial.
/// 
/// This function should be executed when you want to extend the trial period.
/// 
/// The function is only meant for local(unverified) trials.
/// 
/// # Arguments
/// 
/// * `trial_extension_length` - number of days to extend the trial
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the local trial was extended successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn extend_local_trial(trial_extension_length: u32) -> Result<LexActivatorStatus, LexActivatorError> {
    let c_trial_extension_length: c_uint = trial_extension_length as c_uint;
    let status = unsafe { ExtendLocalTrial(c_trial_extension_length) };
    match status {
        0 => Ok(LexActivatorStatus::LA_OK),
        1 => Ok(LexActivatorStatus::LA_FAIL),
        _ => Err(LexActivatorError::from(status)),
    }
}

/// Increments the meter attribute uses of the activation.
/// 
/// # Arguments
/// 
/// * `name` - name of the meter attribute
/// * `increment` - number of units to increment the usage by
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the meter attribute uses was incremented successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn increment_activation_meter_attribute_uses(name: String, increment: u32) -> Result<(), LexActivatorError> {
    let status: i32;
    let c_increment: c_uint = increment as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { IncrementActivationMeterAttributeUses(c_name.as_ptr(), c_increment) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name)?;
        status = unsafe { IncrementActivationMeterAttributeUses(c_name.as_ptr(), c_increment) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Decrements the meter attribute uses of the activation.
/// 
/// # Arguments
/// 
/// * `name` - name of the meter attribute
/// * `decrement` - number of units to decrement the usage by
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the meter attribute uses was decremented successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn decrement_activation_meter_attribute_uses(name: String, decrement: u32) -> Result<(), LexActivatorError> {
    let status: i32;
    let c_decrement: c_uint = decrement as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { DecrementActivationMeterAttributeUses(c_name.as_ptr(), c_decrement) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name)?;
        status = unsafe { DecrementActivationMeterAttributeUses(c_name.as_ptr(), c_decrement) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Resets the meter attribute uses of the activation.
/// 
/// # Arguments
/// 
/// * `name` - name of the meter attribute
/// 
/// # Returns
/// 
/// Returns `Ok(LexActivatorStatus)` with the status code `LexActivatorStatus::LA_OK` if the meter attribute uses was reset successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned. 

pub fn reset_activation_meter_attribute_uses(name: String) -> Result<(), LexActivatorError> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { ResetActivationMeterAttributeUses(c_name.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name)?;
        status = unsafe { ResetActivationMeterAttributeUses(c_name.as_ptr()) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}

/// Resets the activation and trial data stored in the machine.
/// 
/// This function is meant for developer testing only.
/// 
/// The function does not reset local(unverified) trial data.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the activation and trial data was reset successfully. If an error occurs, an `Err` containing the `LexActivatorError`is returned.

pub fn reset() -> Result<(), LexActivatorError> {
    let status = unsafe { Reset() };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorError::from(status));
    }
}
