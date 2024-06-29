import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../utils/cookies';

/** @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/** @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/** @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/


/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
*/
const postPasswordResetPassword = async(password, passwordConfirmation) => {
  let password_reset_response_token = getCookie("password_reset_verification_token");
  if (password_reset_response_token == null) {
      password_reset_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/password-reset/password", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "password_reset_password_token": password_reset_response_token,
    },
    body: JSON.stringify({
      "password": password(),
      "password_confirmation": passwordConfirmation(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} password The user's new password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's new password
*/
const postPassword = async(password, passwordConfirmation) => {
  /** @type {Promise<number|void|Response>} */
  let response = postPasswordResetPassword(password, passwordConfirmation)
    .then((res) => {
      setCookie("password_reset_password_token", res.password_reset_password_token, 1800);
      deleteCookie("password_reset_email_token");
    }) 

  return response;
};

const PasswordResetPasswordForm = () => {
  /** @type {Signal<String>} */
  const [password, setPassword] = createSignal("");
  const [passwordConfirmation, setPasswordConfirmation] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    let response = postPassword(password, passwordConfirmation);
    response.then((response) => console.log("response: ", response));
  }
  
  return (
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
  )
}

export default PasswordResetPasswordForm;
