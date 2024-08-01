import { setCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
const { Request: registerRequest } = require('../../../protos/accounts/password_reset/email/request_pb');
const { Response: registerResponse } = require('../../../protos/accounts/password_reset/email/response_pb');

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
  * @property {Function} verificationMode - go to next screen
*/



/** @param {Accessor<string>} email The user's email address */
const postPasswordResetEmail = async(email) => {
  const request = new registerRequest();
  request.setEmail(email());
  const Buffer = request.serializeBinary();

  const response = await fetch("http://127.0.0.1:8000/password-reset/email", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
    body: Buffer
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} email The user's email address
  * @param {props} props
*/
const postPasswordReset = async(email, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postPasswordResetEmail(email)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);

        let response, token, error;
        try {
            response = registerResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasSuccess()) {
                token = response.getToken();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }
        
      if (token.length == 25) {
          setCookie("password_reset_email_token", token, 5);
          props.verificationMode();
      }
    }) 

  return response;
};


/** @param {props} props */
const PasswordResetEmailForm = (props) => {
  const {email, setEmail} = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    postPasswordReset(email, props);
  }

  return (
    <form onSubmit={PostPasswordReset}>
      <input
        type="email"
        placeholder="email"
        onInput={e => setEmail(e.target.value)}
        value={email()}
        required
      />
      <input type="submit" value="Continue" />
    </form>
  )
}

export default PasswordResetEmailForm;
