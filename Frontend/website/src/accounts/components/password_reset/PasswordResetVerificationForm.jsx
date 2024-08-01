import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
const { Request: passwordResetRequest } = require('../../../protos/accounts/password_reset/verification/request_pb');
const { Response: passwordResetResponse } = require('../../../protos/accounts/password_reset/verification/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the first screen
  * @property {Function} passwordMode - go to next screen
*/

/** @param {Accessor<string>} token The user's token */
const postPasswordResetVerification = async(token) => {
  const request = new passwordResetRequest();
  request.setVerificationCode(token());
  const Buffer = request.serializeBinary();

  let password_reset_response_token = getCookie("password_reset_email_token");
  if (password_reset_response_token == null) {
      password_reset_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/password-reset/verify", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "password_reset_email_token": password_reset_response_token,
    },
    body: Buffer
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} token The user's token
  * @param {props} props
*/
const postPasswordReset = async(token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postPasswordResetVerification(token)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);

        let response, token, error;
        try {
            response = passwordResetResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasToken()) {
                token = response.getToken();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }
        
      if (token.length == 25) {
          setCookie("password_reset_verification_token", token, 1800);
          deleteCookie("password_reset_email_token"); 
          props.passwordMode();
      }
    }) 

  return response;
};

/** @param {props} props */
const PasswordResetVerificationForm = (props) => {
  /** @type {Signal<String>} */
  const [token, setToken] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    console.log("token: ", token());

    let response = postPasswordReset(token, props);
    response.then((response) => console.log("response: ", response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>
      <form onSubmit={PostPasswordReset} >
        <input
          type="text"
          placeholder="token"
          onInput={e => setToken(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>

    </>
  );
};

export default PasswordResetVerificationForm;
