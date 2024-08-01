import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
const { Request: loginRequest } = require('../../../protos/accounts/login/totp/request_pb');
const { Response: loginResponse, Error: loginError } = require('../../../protos/accounts/login/totp/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the first screen
*/

/**
  * @param {number} digit1 totp's first digit
  * @param {number} digit2 totp's second digit
  * @param {number} digit3 totp's third digit
  * @param {number} digit4 totp's fourth digit
  * @param {number} digit5 totp's fith digit
  * @param {number} digit6 totp's sixth digit
*/
const postLoginTotp = async(digit1, digit2, digit3, digit4, digit5, digit6) => {
  const request = new loginRequest();
  request.setDigit1(digit1);
  request.setDigit2(digit2);
  request.setDigit3(digit3);
  request.setDigit4(digit4);
  request.setDigit5(digit5);
  request.setDigit6(digit6);

  const Buffer = request.serializeBinary();
  let login_response_token = getCookie("login_password_token");
  if (login_response_token == null) {
      login_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/login/totp", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "login_password_token": login_response_token,
    },
    body: Buffer
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<number?>} digit1 totp's first digit
  * @param {Accessor<number?>} digit2 totp's second digit
  * @param {Accessor<number?>} digit3 totp's third digit
  * @param {Accessor<number?>} digit4 totp's fourth digit
  * @param {Accessor<number?>} digit5 totp's fith digit
  * @param {Accessor<number?>} digit6 totp's sixth digit
  * @param {Function} setEmail Function to change the email
*/
const postLogin = async(digit1, digit2, digit3, digit4, digit5, digit6, setEmail) => {
  if (
      digit1() == null || digit2() == null || digit3() == null ||
      digit4() == null || digit5() == null || digit6() == null
  ) {
    console.error("digits must be numbers");
  } else {
      /** @type {Promise<void|Response>} */
      let response = postLoginTotp(
          digit1() ?? 0,
          digit2() ?? 0,
          digit3() ?? 0,
          digit4() ?? 0,
          digit5() ?? 0,
          digit6() ?? 0
      )
      .then((array_buffer) => {
          let uint8Array = new Uint8Array(array_buffer);

          let response, error, response_token, access_token, refresh_token, requires_totp;
          try {
              response = loginResponse.deserializeBinary(uint8Array);
              error = response.getError();
              if (response.hasSuccess()) {
                  response_token = response.getToken().getResponse();
                  access_token = response.getToken().getTokens().getAccess();
                  refresh_token = response.getToken().getTokens().getRefresh();
                  requires_totp = response.getRequiresTotp();
              }
          } catch (decodeError) {
              console.error("Error decoding response:", decodeError);
              throw decodeError;
          }

          if (response.hasSuccess()) {
              deleteCookie("login_email_token");
              setEmail("");

              let bearer_token = "Bearer " + access_token;
              setCookie("Authorization", bearer_token, 1800);

              if (refresh_token !== "") {
                  setCookie("Refresh", refresh_token, 18000);
              }
          }
      }) 

      return response;
  }
};

/** @param {props} props */
const LoginTotpForm = (props) => {
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit1, setDigit1] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit2, setDigit2] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit3, setDigit3] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit4, setDigit4] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit5, setDigit5] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit6, setDigit6] = createSignal(null);
  let {setEmail} = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    console.log("digit1", digit1());
    console.log("digit2", digit2());
    console.log("digit3", digit3());
    console.log("digit4", digit4());
    console.log("digit5", digit5());
    console.log("digit6", digit6());

    let response = postLogin(digit1, digit2, digit3, digit4, digit5, digit6, setEmail);
    response.then((response) => console.log("response: ", response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostLogin} >
        <input
          type="text"
          onInput={e => setDigit1(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={e => setDigit2(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={e => setDigit3(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={e => setDigit4(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={e => setDigit5(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={e => setDigit6(+e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default LoginTotpForm;
