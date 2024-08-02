import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { getCookie, deleteCookie } from '../../../utils/cookies';
const { Request: passwordResetRequest } = require('../../../protos/accounts/password_reset/password/request_pb');
const { Response: passwordResetResponse } = require('../../../protos/accounts/password_reset/password/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the first screen
*/


/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
*/
const postPasswordResetPassword = async(password, passwordConfirmation) => {
  const request = new passwordResetRequest();
  request.setPassword(password());
  request.setPasswordConfirmation(passwordConfirmation());
  const Buffer = request.serializeBinary();

  let password_reset_response_token = getCookie("password_reset_verification_token");
  if (password_reset_response_token == null) {
      password_reset_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/password-reset/password", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/x-protobuf",
      "password-Reset-Verification-Token": password_reset_response_token,
    },
    body: Buffer
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} password The user's new password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's new password
  * @param {any} navigate
*/
const postPassword = async(password, passwordConfirmation, navigate) => {
  postPasswordResetPassword(password, passwordConfirmation)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);

        let response, success, error;
        try {
            response = passwordResetResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasSuccess()) {
                success = response.getSuccess();
                deleteCookie("password_reset_verification_token");
                navigate("/login/", { replace: true });
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }
    }) 
};

/** @param {props} props */
const PasswordResetPasswordForm = (props) => {
  /** @type {Signal<String>} */
  const [password, setPassword] = createSignal("");
  const [passwordConfirmation, setPasswordConfirmation] = createSignal("");
  const navigate = useNavigate();

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    let response = postPassword(password, passwordConfirmation, navigate);
    response.then((response) => console.log("response: ", response));
  }
  
  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostPasswordReset} >
        <input
          type="password"
          placeholder="password confirmation"
          onInput={e => setPassword(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password confirmation"
          onInput={e => setPasswordConfirmation(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  )
}

export default PasswordResetPasswordForm;

