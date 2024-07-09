import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
  * @property {Function} emailMode - go to the first screen
*/

/**
  * @param {Accessor<string>} totp The user's totp
  */
const postLoginTotp = async(totp) => {
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
    body: JSON.stringify({
      "totp": totp(),
    })
  });

  return response.json();
}

/**
  * @param {Accessor<string>} totp The user's totp
*/
const postLogin = async(totp) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginTotp(totp)
    .then((res) => {
      console.log("response: \n");
      console.log(res);
      console.log(res.account_error.is_error);
      if (res.account_error.is_error == false) {
        deleteCookie("login_email_token");

        let {setEmail} = useEmailContext();
        setEmail("");

        if (res.is_error == false) {
          let bearer_token = "Bearer " + res.auth_tokens.access_token;
          setCookie("Authorization", bearer_token, 1800);

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
const LoginTotpForm = (props) => {
  /** @type {Signal<String>} */
  const [totp, setTotp] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
    e.preventDefault();
    console.log("totp", totp());

    let response = postLogin(totp);
    response.then((response) => console.log("response: ", response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>x</button>

      <form onSubmit={PostLogin} >
        <input
          type="text"
          placeholder="totp"
          onInput={e => setTotp(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default LoginTotpForm;
