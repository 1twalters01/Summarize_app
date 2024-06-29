import { createSignal } from 'solid-js';
import { setCookie } from '../../utils/cookies';

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
  * @property {Function} verificationMode - go to next screen
*/



/** @param {Accessor<string>} email The user's email address */
const postPasswordResetEmail = async(email) => {
  const response = await fetch("http://127.0.0.1:8000/password-reset/email", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      "email": email(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} email The user's email address
  * @param {props} props
*/
const postPasswordReset = async(email, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postPasswordResetEmail(email)
    .then((res) => {
      if (res.password_reset_response_token != null) {
        setCookie("password_reset_email_token", res.password_reset_response_token, 5);
        props.verificationMode();
      }
    }) 

  return response;
};


/** @param {props} props */
const PasswordResetEmailForm = (props) => {
  /** @type {Signal<String>} */
  const [email, setEmail] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    postPasswordReset(email, props);
  }


  return (
    <form onSubmit={PostPasswordReset} >
      <input
        type="email"
        placeholder="email"
        onInput={e => setEmail(e.target.value)}
        required
      />
      <input type="submit" value="Login" />
    </form>
  )
}

export default PasswordResetEmailForm;
