/**
  * @param {Accessor<string>} password The user's password 
  * @param {props} props
*/
const postLogin = async(password, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postLoginPassword(password)
    .then((res) => {
      let login_response_token = res.login_response_token;
      if (login_response_token != null) {
        setCookie("login_password_token", login_response_token, 5);
        props.totpMode();
      }
    }) 

  return response;
};

