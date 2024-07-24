import { useEmailContext } from '../../context/EmailContext';
import { setCookie } from '../../../utils/cookies';
import { accounts as accountsRequest } from '../../../protos/accounts/login/email/request';
import { accounts as accountsResponse } from '../../../protos/accounts/login/email/response';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} passwordMode - go to next screen
*/

const Request = accountsRequest?.login?.email?.request?.Request;
const Response = accountsResponse?.login?.email?.response?.Response;
console.log(accountsResponse);
console.log(accountsResponse.login);
console.log(accountsResponse.login.email);
console.log(accountsResponse.login.email.response);
console.log(Response);

/** @param {Accessor<string>} email The user's email address */
const postLoginEmail = async(email) => {
  const message = {email: email()};
  const Buffer = Request.encode(message).finish();

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
        
        const testMessage = { token:"fjh83hwefh9803igr5s34d6gy" };
        console.log("Test message:", testMessage);
        let encoded = Response.encode(testMessage).finish();
        console.log("Encoded message:", encoded);
        let decoded = Response.decode(encoded);
        console.log("Decoded message:", decoded);

        let response;
        try {
            response = Response.decode(uint8Array);
            console.log("Decoded response:", response);
        } catch (decodeError) {
            console.log("oof");
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }

      // let response = Response.decode(uint8Array);
        // console.log(response);
      if ("token" in response) {
          setCookie("login_email_token", /** @type String */ (response.token), 5);
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

