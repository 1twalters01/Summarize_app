import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../utils/cookies';

/** @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/** @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/** @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/


/**
  * @param {Accessor<string>} username The user's username
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
  * @param {Accessor<string>} firstName The user's first name (optional)
  * @param {Accessor<string>} lastName The user's last name (optional)
*/
const postRegisterDetails = async(username, password, passwordConfirmation, firstName, lastName) => {
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
    body: JSON.stringify({
      "username": username(),
      "password": password(),
      "password_confirmation": passwordConfirmation(),
      "first_name": firstName(),
      "last_name": lastName(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} username The user's username
  * @param {Accessor<string>} password The user's password
  * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
  * @param {Accessor<string>} firstName The user's first name (optional)
  * @param {Accessor<string>} lastName The user's last name (optional)
*/
const postDetails = async(username, password, passwordConfirmation, firstName, lastName) => {
  /** @type {Promise<number|void|Response>} */
  let response = postRegisterDetails(username, password, passwordConfirmation, firstName, lastName)
    .then((res) => {
      deleteCookie("register_verify_token");
    })
};

const RegisterDetailsForm = () => {
  /** @type {Signal<String>} */
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [passwordConfirmation, setPasswordConfirmation] = createSignal("");
  const [firstName, setFirstName] = createSignal("");
  const [lastName, setLastName] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    postDetails(username, password, passwordConfirmation, firstName, lastName);
    const navigate = useNavigate();
    navigate('/login');
  }

  return (
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
  );
};

export default RegisterDetailsForm;



