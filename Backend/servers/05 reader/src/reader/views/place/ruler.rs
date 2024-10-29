// turn on/off ruler
// Move to settings?
// specific to account or to device or to device type

// Priority Device > Device Type > Account
async pub fn post_retrieve_toggle_ruler_status(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { device, device_type } = data.0;
    // Check if device in pg in true of false vec
    // if true then return
    // Check if device type in pg in true of false vec
    // if true then return
    // Check in account
    // return val
}

async pub fn post_toggle_ruler_account(data: Protobuf<request>) -> Result<impl Responder> {
    // state = bool
    let Request { state } = data.0;

    // update account ruler status in pg with status
    // if error return error
    // return ok
}

async pub fn post_toggle_ruler_device_type(data: Protobuf<request>) -> Result<impl Responder> {
    let Request { state, device_type } = data.0

    // update device type ruler status in pg with status and device type
        // Add or remove from a vec for if status is true?
        // This will capture for none case as well
    // if error return error
    // return ok
}

async pub fn post_toggle_ruler_device(data: Protobuf<request>) -> Result<impl Responder> {
    // update device ruler status in pg with status and device id
        // Add or remove from a vec for if status is true?
        // Have another for false?
        // This will capture for none case as well
    // if error return error
    // return ok
}