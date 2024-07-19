import { createSignal } from 'solid-js';
import { A } from '@solidjs/router';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
import { encodeRequest } from '../../../protos/accounts/login/password_request';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the email screen
  * @property {Function} totpMode - go to next screen
*/



/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<boolean>} rememberMe The user's remember me status
  */
const postLoginPassword = async(password, rememberMe) => {
  const Buffer = encodeRequest({password: password(), remember_me: rememberMe()})

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

  return response.json();
}

/**
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<boolean>} rememberMe The user's remember me status
  * @param {props} props
  */
const postLogin = async(password, rememberMe, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginPassword(password, rememberMe)
    .then((res) => {
      console.log("response: \n");
      console.log(res);
      console.log(res.account_error.is_error);
      if (res.account_error.is_error == false) {
        deleteCookie("login_email_token");

        if (res.has_totp == true) {
          setCookie("login_password_token", res.login_response_token, 1800);
          props.totpMode();
        } else {
          let bearer_token = "Bearer " + res.auth_tokens.access_token;
          setCookie("Authorization", bearer_token, 1800);

          let {setEmail} = useEmailContext();
          setEmail("");

          let refresh_token = res.auth_tokens.refresh_token;
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
