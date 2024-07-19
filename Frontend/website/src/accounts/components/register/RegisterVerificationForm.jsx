import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T * @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to first screen
  * @property {Function} detailsMode - go to the next screen
*/

/** @param {Accessor<string>} token The user's token */
const postRegisterVerification = async(token) => {
  let register_response_token = getCookie("register_email_token");
  if (register_response_token == null) {
      register_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/register/verify", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "register_email_token": register_response_token,
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
const postRegister = async(token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postRegisterVerification(token)
    .then((res) => {
      if (res.register_response_token != null) {
        setCookie("register_verification_token", res.register_response_token, 1800);
        deleteCookie("register_email_token"); 
        props.detailsMode();
      }
    }) 

  return response;
};

/** @param {props} props */
const RegisterVerificationForm = (props) => {
  /** @type {Signal<String>} */
  const [token, setToken] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    console.log("token: ", token());

    let response = postRegister(token, props);
    response.then((response) => console.log("response: ", response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostRegister} >
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

export default RegisterVerificationForm;

