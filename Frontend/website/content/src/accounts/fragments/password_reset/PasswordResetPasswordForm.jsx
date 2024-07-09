import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { getCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';

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
  let password_reset_response_token = getCookie("password_reset_verification_token");
  if (password_reset_response_token == null) {
      password_reset_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/password-reset/password", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "password_reset_verification_token": password_reset_response_token,
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
  postPasswordResetPassword(password, passwordConfirmation)
    .then((res) => {
      if (res.account_error.is_error == false) {
        deleteCookie("password_reset_verification_token");
        
        let {setEmail} = useEmailContext();
        setEmail("");
        
        const navigate = useNavigate();
        navigate("/login", { replace: true });
      }

    }) 
};

/** @param {props} props */
const PasswordResetPasswordForm = (props) => {
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

