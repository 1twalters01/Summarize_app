import { createSignal } from 'solid-js';
import { A } from '@solidjs/router';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
const { Request } = require('../../../protos/accounts/login/password/request_pb');
// import { accounts as accountsResponse } from '../../../protos/accounts/login/password/response';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the email screen
  * @property {Function} totpMode - go to next screen
*/


// let Request = accountsRequest.login.password.request.Request;
let Response = Request;
// let Response = accountsResponse.login.password.response.Response;

/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<boolean>} rememberMe The user's remember me status
  */
const postLoginPassword = async(password, rememberMe) => {
  const message = {password: password(), rememberMe: rememberMe()};
  const Buffer = Request.encode(message).finish();

  let login_response_token = getCookie("login_email_token");
  if (login_response_token == null) {
      login_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/login/password", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/x-protobuf",
      "login_email_token": login_response_token,
    },
    body: Buffer 
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<boolean>} rememberMe The user's remember me status
  * @param {props} props
  */
const postLogin = async(password, rememberMe, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginPassword(password, rememberMe)
    .then((array_buffer) => {
      let uint8array = new Uint8Array(array_buffer);
      console.log("hi");
      let response = Response.decode(uint8array);
      console.log("response: ", response);
      if ("token" in response && "requires_totp" in response) {
        deleteCookie("login_email_token");

        let token = (response.token);
        let requires_totp = response.requires_totp;

        if (requires_totp == true) {
          setCookie("login_password_token", /** @type String */ (token.response), 1800);
          props.totpMode();
        } else {
          let tokens = token.tokens;
          let bearer_token = "Bearer " + /** @type String */ (tokens.access);
          setCookie("Authorization", bearer_token, 1800);

          let {setEmail} = useEmailContext();
          setEmail("");

          let refresh_token = tokens.refresh;
          if (refresh_token != null) {
              setCookie("Refresh", refresh_token, 18000)
          }
        }
      }
    }) 

  return response;
};

/** @param {props} props */
const LoginPasswordForm = (props) => {
  /** @type {Signal<String>} */
  const [password, setPassword] = createSignal("");
  const [rememberMe, setRememberMe] = createSignal(false);

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    console.log("password: ", password());

    let response = postLogin(password, rememberMe, props);
    response.then((response) => console.log("response: ", response));
  }
  
  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostLogin} >
        <input
          type="password"
          placeholder="password"
          onInput={e => setPassword(e.target.value)}
          required
        />
        <input
          type="checkbox"
          checked={rememberMe()}
          onChange={e => setRememberMe(e.target.checked)}
        />
        <input type="submit" value="Submit" />
      </form>

      <A href="/password-reset">Forgot your password?</A>
    </>
  );
};

export default LoginPasswordForm;
