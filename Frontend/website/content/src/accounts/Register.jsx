import { createSignal } from 'solid-js';

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

/** @param {Accessor<string>} email The user's email address */
const postRegisterEmail = async(email) => {
    const response = await fetch("http://127.0.0.1:8000/register/email", {
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

/** @param {Accessor<string>} email The user's email address
 */
const postRegister = async(email) => {
    /** @type {Promise<number|void|Response>} */
    let response;
    response = postRegisterEmail(email)
        .then((res) => {
            console.log(res);
        }) 

    return response;
};

const Register = () => {
  /** @type {Signal<String>} */
  const [email, setEmail] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
      e.preventDefault();
      console.log("email: ", email());

      // json
      let response = postRegister(email);
      response.then((response) => console.log("response: ", response));
  }

  return (
    <>
      <h1>Register</h1>

      <form onSubmit={PostRegister} >
        <input
          type="email"
          placeholder="email"
          onInput={e => setEmail(e.target.value)}
          required
        />
        <input type="submit" value="Login" />
      </form>
    </>
  );
};

export default Register;

