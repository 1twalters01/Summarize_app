import { useContext, createSignal } from 'solid-js';
import { EmailContext } from '../../context/EmailContext';
import { setCookie } from '../../../utils/cookies';

// import { A } from '@solidjs/router';

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
  * @property {Function} passwordMode - go to next screen
*/


/** @param {Accessor<string>} email The user's email address */
const postLoginEmail = async(email) => {
  const response = await fetch("http://127.0.0.1:8000/login/email", {
    method: "POST",
    mode: "cors",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      "email": email(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} email The user's email address
  * @param {props} props
*/
const postLogin = async(email, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginEmail(email)
    .then((res) => {
      console.log(res)
      let login_response_token = res.login_response_token;
      if (login_response_token != null) {
        setCookie("login_email_token", login_response_token, 5);
        console.log("works");
        props.passwordMode();
      }
    }) 

  return response;
};


/** @param {props} props */
const LoginEmailFormFragment = (props) => {
  const {email, setEmail} = useContext(EmailContext);
  // const [email, setEmail] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    postLogin(email, props);
  }

  return (
    <>
      {email()}
      <form onSubmit={PostLogin} >
        <input
          type="email"
          placeholder="email"
          onInput={e => setEmail(e.target.value)}
          value={email()}
          required
        />
        <input type="submit" value="Login" />
      </form>
    </>
  );
};

export default LoginEmailFormFragment;
