import { createSignal } from 'solid-js';
// const Navbar = lazy(() => import('../navbar'));
import Navbar from '../navbar';
// import styles from './Accounts.module.css';

/** 
  * @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/**
  * @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/**
  * @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/


/**
  * Enum for mode values.
  * @readonly
  * @enum {number}
  */
var modeOptions = {
  email: 0,
  password: 1,
  totp: 2,
};

/**
 ** @param {Accessor<String>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 ** @param {Accessor<string>} totp The user's totp
 */
const validateModeEmail = (email, password, totp) => {
  if (email() == "") {
      throw new Error("Email not supported for email login");
  }
  if (password() != "") {
    throw new Error("Password not supported for email login");
  }

  if (totp() != "") {
    throw new Error("Totp not supported for email login");
  }
}

/**
 ** @param {Accessor<String>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 ** @param {Accessor<string>} totp The user's totp
 */
const validateModePassword = (email, password, totp) => {
  if (email() == "") {
      throw new Error("Email not supported for password login");
  }
  if (password() == "") {
    throw new Error("Password required for password login");
  }

  if (totp() != "") {
    throw new Error("Totp not supported for password login");
  }
}

/**
 ** @param {Accessor<String>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 ** @param {Accessor<string>} totp The user's totp
 */
const validateModeTotp = (email, password, totp) => {
  if (email() == "") {
      throw new Error("Email not supported for password login");
  }
  if (password() == "") {
    throw new Error("Password required for password login");
  }

  if (totp() == "") {
    throw new Error("Totp required for password login");
  }
}

/**
 ** @param {Accessor<modeOptions>} mode mode getter
 ** @param {Setter<modeOptions>} setMode mode setter
 ** @param {Accessor<string>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 ** @param {Accessor<string>} totp The user's totp
 */
const postLogin = async(mode, setMode, email, password, totp) => {
    /** @type {Promise<number|void|Response>} */
    let response;
    if (mode() === modeOptions.email) {
      validateModeEmail(email, password, totp);
      response = postLoginEmail(email)
        .then((res) => {
            console.log(res);
            if (res === true) {
                setMode(modeOptions.password)
            }});
    } else if (mode() === modeOptions.password) {
      validateModePassword(email, password, totp);
      response = postLoginPassword(email, password)
        .then((res) => {
            if (res === true) {
                setMode(modeOptions.totp)
        }});

    } else if (mode() === modeOptions.totp) {
      validateModeTotp(email, password, totp);
      response = postLoginTotp(email, password, totp);
    } else {
      throw new Error("Invalid mode: " + mode);
    }
    
    return response;
};

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
 ** @param {Accessor<string>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 */
const postLoginPassword = async(email, password) => {
    const response = await fetch("http://127.0.0.1:8000/login/password", {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "email": email(),
            "password": password(),
        })
    });

    return response.json();
}

/**
 ** @param {Accessor<string>} email The user's email address
 ** @param {Accessor<string>} password The user's password
 ** @param {Accessor<string>} totp The user's totp
 */
const postLoginTotp = async(email, password, totp) => {
    /** @type {Response} */
    const response = await fetch("http://127.0.0.1:8000/login/totp", {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "email": email(),
            "password": password(),
            "totp": totp(),
        })
    });

    return response;
}

const Login = () => {
  /** @type {Signal<modeOptions>} */
  const [mode, setMode] = createSignal(modeOptions.email);
  /** @type {Signal<String>} */
  const [email, setEmail] = createSignal("");
  /** @type {Signal<string>} */
  const [password, setPassword] = createSignal("");
  /** @type {Signal<string>} */
  const [totp, setTotp] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
      e.preventDefault();
      console.log("email: ", email(), "\npassword: ", password());

      // json
      let response = postLogin(mode, setMode, email, password, totp);
      response.then((response) => console.log("response: ", response));
  }

  return (
    // xsrf token?
    <>
      <Navbar />
      
      <h1>Log in</h1>
      <form onSubmit={PostLogin} >
        <input
          type="email"
          placeholder="email"
          onInput={e => setEmail(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password"
          onInput={e => setPassword(e.target.value)}
        />
        <input
          type="input"
          placeholder="totp"
          onInput={e => setTotp(e.target.value)}
        />
        <input type="submit" value="Login" />
      </form>
      <p>{email()}</p>
    </>
  );
};

export default Login;

