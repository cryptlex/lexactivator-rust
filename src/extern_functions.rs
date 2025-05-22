//https://doc.rust-lang.org/std/os/raw/index.html#types
use std::ffi::{c_char, c_int, c_uint, c_longlong, c_ulonglong};

use crate::LexActivatorCode;

#[cfg(windows)]
macro_rules! cstrtype {
    () => {
        *const u16
    };
}

#[cfg(not(windows))]
macro_rules! cstrtype {
    () => {
        *const c_char
    };
}

#[cfg(windows)]
macro_rules! strtype {
    () => {
        *mut u16
    }
}

#[cfg(not(windows))]
macro_rules! strtype {
    () => {
        *mut c_char
    }
}

pub type CallbackType = extern "C" fn(i32);

extern "C" {
    // --------------- Setter Functions ---------------
    pub fn SetProductData(productData: cstrtype!()) -> c_int;
    pub fn SetProductId(productId: cstrtype!() , flags: c_uint) -> c_int;
    pub fn SetDataDirectory(dataDir: cstrtype!()) -> c_int;
    pub fn SetDebugMode(enable: c_uint) -> c_int;
    pub fn SetCacheMode(mode: c_uint) -> c_int;
    pub fn SetCustomDeviceFingerprint(deviceFingerprint: cstrtype!()) -> c_int;
    pub fn SetLicenseKey(licenseKey: cstrtype!()) -> c_int;
    pub fn SetLicenseUserCredential(email: cstrtype!(), password: cstrtype!()) -> c_int;
    pub fn SetLicenseCallback(callback: CallbackType) ->c_int;
    pub fn SetActivationLeaseDuration(leaseDuration: c_longlong) -> c_int;
    pub fn SetActivationMetadata(key: cstrtype!(), value: cstrtype!()) -> c_int;
    pub fn SetTrialActivationMetadata(key: cstrtype!(), value: cstrtype!()) -> c_int;
    pub fn SetReleaseVersion(releaseVersion: cstrtype!()) -> c_int;
    pub fn SetReleasePublishedDate(releasePublishedDate: c_uint) -> c_int;
    pub fn SetReleasePlatform(platform: cstrtype!()) -> c_int;
    pub fn SetReleaseChannel(channel: cstrtype!()) -> c_int;
    pub fn SetOfflineActivationRequestMeterAttributeUses(name: cstrtype!(), uses: c_uint) -> c_int;
    pub fn SetNetworkProxy(proxy: cstrtype!()) -> c_int;
    pub fn SetCryptlexHost(host: cstrtype!()) -> c_int;
    pub fn SetTwoFactorAuthenticationCode(twoFactorAuthenticationCode: cstrtype!()) -> c_int;

    // --------------- Getter Functions ---------------

    pub fn GetProductMetadata(key: cstrtype!(), value: strtype!(), length: c_uint) -> c_int;
    pub fn GetProductVersionName(name: strtype!(), length: c_uint) -> c_int;
    pub fn GetProductVersionDisplayName(name: strtype!(), length: c_uint) -> c_int;
    pub fn GetProductVersionFeatureFlag(name: cstrtype!(), enabled: *mut c_uint, data: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseMetadata(key: cstrtype!(), value: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseKey(licenseKey: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseAllowedActivations(allowedActivations: *mut c_longlong) -> c_int;
    pub fn GetLicenseAllowedDeactivations(allowedDeactivations: *mut c_longlong) -> c_int;
    pub fn GetLicenseTotalActivations(totalActivations: *mut c_uint) -> c_int;
    pub fn GetLicenseTotalDeactivations(totalDeactivations: *mut c_uint) -> c_int;
    pub fn GetLicenseCreationDate(creationDate: *mut c_uint) -> c_int;
    pub fn GetLicenseActivationDate(activationDate: *mut c_uint) -> c_int;
    pub fn GetActivationLastSyncedDate(lastSyncedDate: *mut c_uint) -> c_int;
    pub fn GetLicenseExpiryDate(expiryDate: *mut c_uint) -> c_int;
    pub fn GetLicenseMaintenanceExpiryDate(maintenanceExpiryDate: *mut c_uint) -> c_int;
    pub fn GetLicenseMaxAllowedReleaseVersion(maxAllowedReleaseVersion: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseUserEmail(email: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseUserName(name: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseUserCompany(company: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseUserMetadata(key: cstrtype!(), value: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseOrganizationName(organizationName: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseOrganizationAddressInternal(organizationAddressJson: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseEntitlementSetName(name: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseEntitlementSetDisplayName(displayName: strtype!(), length: c_uint) -> c_int;
    pub fn GetFeatureEntitlementsInternal(featureEntitlementsJson: strtype!(), length: c_uint) -> c_int;
    pub fn GetFeatureEntitlementInternal(featureName: cstrtype!(), featureEntitlementJson: strtype!(), length: c_uint) -> c_int;
    pub fn GetUserLicensesInternal(userLicenses: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseType(licenseType: strtype!(), length: c_uint) -> c_int;
    pub fn GetActivationId(id:strtype!(), length: c_uint) -> c_int;
    pub fn GetActivationMetadata(key: cstrtype!(), value: strtype!(), length: c_uint) -> c_int;
    pub fn GetActivationMode(initialMode: strtype!(), initialModeLength: c_uint, currentMode: strtype!(), currentModeLength: c_uint) -> c_int;
    pub fn GetActivationMeterAttributeUses(name: cstrtype!(), uses: *mut c_uint) -> c_int;
    pub fn GetServerSyncGracePeriodExpiryDate(gracePeriodExpiryDate: *mut c_uint) -> c_int;
    pub fn GetTrialActivationMetadata(key: cstrtype!(), value: strtype!(), length: c_uint) -> c_int;
    pub fn GetTrialExpiryDate(trialExpiryDate: *mut c_uint) -> c_int;
    pub fn GetTrialId(trialId: strtype!(), length: c_uint) -> c_int;
    pub fn GetLocalTrialExpiryDate(localTrialExpiryDate: *mut c_uint) -> c_int;
    pub fn GetLibraryVersion(libraryVersion: strtype!(), length: c_uint) -> c_int;
    pub fn GetLicenseMeterAttribute(name: cstrtype!(), allowedUses: *mut c_longlong, totalUses: *mut c_ulonglong, grossUses: *mut c_ulonglong) -> c_int;

    // --------------- LexActivator Action Functions ---------------

    pub fn AuthenticateUser(email: cstrtype!(), password: cstrtype!()) -> c_int;
    pub fn AuthenticateUserWithIdToken(idToken: cstrtype!()) -> c_int;
    pub fn ActivateLicense() -> c_int;
    pub fn ActivateLicenseOffline(filePath: cstrtype!()) -> c_int;
    pub fn GenerateOfflineActivationRequest(filePath: cstrtype!()) -> c_int;
    pub fn DeactivateLicense() -> c_int;
    pub fn GenerateOfflineDeactivationRequest(filePath: cstrtype!()) -> c_int;
    pub fn IsLicenseGenuine() -> c_int;
    pub fn IsLicenseValid() -> c_int;
    pub fn ActivateTrial() -> c_int;
    pub fn ActivateTrialOffline(filePath: cstrtype!()) -> c_int;
    pub fn GenerateOfflineTrialActivationRequest(filePath: cstrtype!()) -> c_int;
    pub fn IsTrialGenuine() -> c_int;
    pub fn ActivateLocalTrial(trialLength: c_uint) -> c_int;
    pub fn IsLocalTrialGenuine() -> c_int;
    pub fn ExtendLocalTrial(trialExtensionLength: c_uint) -> c_int;
    pub fn IncrementActivationMeterAttributeUses(name: cstrtype!(), increment: c_uint) -> c_int;
    pub fn DecrementActivationMeterAttributeUses(name: cstrtype!(), decrement: c_uint) -> c_int;
    pub fn ResetActivationMeterAttributeUses(name: cstrtype!()) -> c_int;
    pub fn Reset() -> c_int;
}