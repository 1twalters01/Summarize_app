import { createSignal } from 'solid-js';
import { setCookie } from '../../../utils/cookies';

/** @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/** @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/** @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/


/** @param {Accessor<string>} totp The user's totp */
const postLoginTotp = async(totp) => {
  const response = await fetch("http://127.0.0.1:8000/login/totp", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      "totp": totp(),
    })
  });

  return response.json();
}


/** @typedef {object} LoginResponse
    * @property {String} login_response_token - string response from server
*/

/**
  * @param {Accessor<string>} totp The user's totp 
  * @param {Function} postLoginFunction The relevent function for post login
*/
const postLogin = async(totp, postLoginFunction) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginFunction(totp)
    .then((/** @type {LoginResponse} */ res) => {
      let login_response_token = res.login_response_token;
      if (login_response_token != null) {
        setCookie("login_totp_token", login_response_token, 5);
      }
    }) 

  return response;
};


const LoginTotpForm = () => {
  /** @type {Signal<String>} */
  const [totp, setTotp] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    postLogin(totp, postLoginTotp);
  }

  return (
    <form onSubmit={PostLogin} >
      <input
        type="number"
        placeholder="totp"
        onInput={e => setTotp(e.target.value)}
        required
      />
      <input type="submit" value="Login" />
    </form>
  );
};

export default LoginTotpForm;


