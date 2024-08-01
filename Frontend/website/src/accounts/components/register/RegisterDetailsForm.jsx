import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { getCookie, deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
const { Request: registerRequest } = require('../../../protos/accounts/register/verification/request_pb');
const { Response: registerResponse } = require('../../../protos/accounts/register/verification/response_pb');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the first screen
*/

/**
  * @param {Accessor<string>} username The user's username
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
  * @param {Accessor<string>} firstName The user's first name (optional)
  * @param {Accessor<string>} lastName The user's last name (optional)
*/
const postRegisterDetails = async(username, password, passwordConfirmation, firstName, lastName) => {
  const request = new registerRequest();
  request.setUsername(username());
  request.setPassword(password());
  request.setPasswordConfirmation(passwordConfirmation());
  request.setFirstName(firstName());
  request.setLastName(lastName());
  const Buffer = request.serializeBinary();

  let register_response_token = getCookie("register_verification_token");
  if (register_response_token == null) {
      register_response_token = "";
  }
  const response = await fetch("http://127.0.0.1:8000/register/details", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
      "register_verification_token": register_response_token,
    },
    body: Buffer
  });

  return response.arrayBuffer();
}

/**
  * @param {Accessor<string>} username The user's username
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
  * @param {Accessor<string>} firstName The user's first name (optional)
  * @param {Accessor<string>} lastName The user's last name (optional)
  * @param {Function} setEmail Function to change the email
*/
const postDetails = async(username, password, passwordConfirmation, firstName, lastName, setEmail) => {
  postRegisterDetails(username, password, passwordConfirmation, firstName, lastName)
    .then((arrayBuffer) => {
        let uint8Array = new Uint8Array(arrayBuffer);
        
        let response, success, error;
        try {
            response = registerResponse.deserializeBinary(uint8Array);
            error = response.getError();
            if (response.hasSuccess()) {
                success = response.getSuccess();
            }
        } catch (decodeError) {
            console.error("Error decoding response:", decodeError);
            throw decodeError;
        }

        if (response.hasSuccess()) {
            deleteCookie("register_verify_token");
            setEmail("");

            const navigate = useNavigate();
            navigate("/login", { replace: true });
        }
    })
};

/** @param {props} props */
const RegisterDetailsForm = (props) => {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [passwordConfirmation, setPasswordConfirmation] = createSignal("");
  const [firstName, setFirstName] = createSignal("");
  const [lastName, setLastName] = createSignal("");
  const {setEmail} = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    postDetails(username, password, passwordConfirmation, firstName, lastName, setEmail);
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostRegister} >
        <input
          type="text"
          placeholder="username"
          onInput={e => setUsername(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password"
          onInput={e => setPassword(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password confirmation"
          onInput={e => setPasswordConfirmation(e.target.value)}
          required
        />
        <input
          type="text"
          placeholder="first name"
          onInput={e => setFirstName(e.target.value)}
        />
        <input
          type="text"
          placeholder="last name"
          onInput={e => setLastName(e.target.value)}
        />
        <input type="submit" value="Login" />
      </form>
    </>
  );
};

export default RegisterDetailsForm;



