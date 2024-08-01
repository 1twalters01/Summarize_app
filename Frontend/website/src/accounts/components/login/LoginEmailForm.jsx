import { useEmailContext } from '../../context/EmailContext';
import { setCookie } from '../../../utils/cookies';
const { Request: loginRequest } = require('../../../protos/accounts/login/email/request_pb');
const { Response: loginResponse } = require('../../../protos/accounts/login/email/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} passwordMode - go to next screen
*/

/** @param {Accessor<string>} email The user's email address */
const postLoginEmail = async(email) => {
  const request = new loginRequest();
  request.setEmail(email());
  const Buffer = request.serializeBinary();

  const response = await fetch("http://127.0.0.1:8000/login/email", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/x-protobuf",
    },
    body: Buffer 
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} email The user's email address
  * @param {props} props
*/
const postLogin = async(email, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginEmail(email)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);
        
        let response, token, error;
        try {
            response = loginResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasSuccess()) {
                token = response.getToken();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }

      if (token.length == 25) {
          setCookie("login_email_token", /** @type String */ (token), 5);
          props.passwordMode();
      }
    }) 

  return response;
};


/** @param {props} props */
const LoginEmailForm = (props) => {
  const {email, setEmail} = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    postLogin(email, props);
  }

  return (
    <>
      <form onSubmit={PostLogin} >
        <input
          type="email"
          placeholder="email"
          onInput={e => setEmail(e.target.value)}
          value={email()}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default LoginEmailForm;

