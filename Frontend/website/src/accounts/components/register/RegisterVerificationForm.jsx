import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
const { Request: registerRequest } = require('../../../protos/accounts/register/verification/request_pb');
const { Response: registerResponse } = require('../../../protos/accounts/register/verification/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to first screen
  * @property {Function} detailsMode - go to the next screen
*/

/** @param {Accessor<string>} token The user's token */
const postRegisterVerification = async(token) => {
  const request = new registerRequest();
  request.setVerificationCode(token());
  const Buffer = request.serializeBinary();

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
    body: Buffer,
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} token The user's token
  * @param {props} props
*/
const postRegister = async(token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postRegisterVerification(token)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);
        
        let response, token, error;
        try {
            response = registerResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasToken()) {
                token = response.getToken();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }

        if (token.length == 25) {
            setCookie("register_verification_token", token, 1800);
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

