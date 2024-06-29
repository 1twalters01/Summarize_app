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

/** @typedef {Object} props
  * @property {Function} passwordMode - go to next screen
*/

/** @param {Accessor<string>} token The user's token */
const postPasswordResetVerification = async(token) => {
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
    body: JSON.stringify({
      "verification_token": token(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} token The user's token
  * @param {props} props
*/
const postPasswordReset = async(token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postPasswordResetVerification(token)
    .then((res) => {
      if (res.password_reset_response_token != null) {
        setCookie("password_reset_verification_token", res.password_reset_response_token, 1800);
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
