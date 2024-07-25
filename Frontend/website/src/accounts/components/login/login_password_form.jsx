import { createSignal } from 'solid-js';
import { A } from '@solidjs/router';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
const { Request: loginRequest } = require('../../../protos/accounts/login/password/request_pb');
const { Response: loginResponse, Error: loginError } = require('../../../protos/accounts/login/password/response_pb');
// import { accounts as accountsResponse } from '../../../protos/accounts/login/password/response';

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
  const request = new loginRequest();
  request.setPassword(password());
  request.setRememberMe(rememberMe());
  const Buffer = request.serializeBinary();

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
  * @param {Function} setEmail Function to change the email
  * @param {props} props
  */
const postLogin = async(password, rememberMe, setEmail, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginPassword(password, rememberMe)
    .then((array_buffer) => {
      let uint8Array = new Uint8Array(array_buffer);
        let response, error, response_token, access_token, refresh_token, requires_totp;
        try {
            response = loginResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasSuccess()) {
              response_token = response.getSuccess().getToken().getResponse();
              access_token = response.getSuccess().getToken().getTokens().getAccess();
              refresh_token = response.getSuccess().getToken().getTokens().getRefresh();
              requires_totp = response.getSuccess().getRequiresTotp();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }
        console.log("response:", response);
        console.log("error code:", error);
        console.log("error name:", loginError[error]);
        console.log("invalid credentials:",loginError.INVALID_CREDENTIALS);
        console.log("response token:", response_token);
        console.log("access token:", access_token);
        console.log("refresh token:", refresh_token);
        console.log("requires totp:", requires_totp);

      if (response.hasSuccess()) {
        deleteCookie("login_email_token");
        setEmail("");

        if (requires_totp == true) {
          setCookie("login_password_token", /** @type String */ (response_token), 1800);
          props.totpMode();
        } else {
          let bearer_token = "Bearer " + access_token;
          setCookie("Authorization", bearer_token, 1800);

          if (refresh_token !== "") {
              setCookie("Refresh", refresh_token, 18000);
          } else {
              console.log("yooo");
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
  const {setEmail} = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    console.log("password: ", password());

    let response = postLogin(password, rememberMe, setEmail, props);
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
