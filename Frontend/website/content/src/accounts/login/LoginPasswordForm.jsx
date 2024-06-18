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
  * @property {Function} totpMode - go to next screen
*/



/** @param {Accessor<string>} password The user's email address */
const postLoginPassword = async(password) => {
  const response = await fetch("http://127.0.0.1:8000/login/totp", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      "password": password(),
    })
  });

  return response.json();
}


/** @typedef {object} LoginResponse
    * @property {String} login_response_token - string response from server
*/

/**
  * @param {Accessor<string>} password The user's password 
  * @param {Function} postLoginFunction The relevent function for post login
  * @param {props} props
*/
const postLogin = async(password, postLoginFunction, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginFunction(password)
    .then((/** @type {LoginResponse} */ res) => {
      let login_response_token = res.login_response_token;
      if (login_response_token != null) {
        setCookie("login_password_token", login_response_token, 5);
        props.totpMode();
      }
    }) 

  return response;
};


/** @param {props} props */
const LoginPasswordForm = (props) => {
  /** @type {Signal<String>} */
  const [password, setPassword] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    postLogin(password, postLoginPassword, props);
  }

  return (
    <form onSubmit={PostLogin} >
      <input
        type="password"
        placeholder="password"
        onInput={e => setPassword(e.target.value)}
        required
      />
      <input type="submit" value="Login" />
    </form>
  );
};

export default LoginPasswordForm;



