import Headers from "../../components/login/headers";
import Oauth2 from "../../components/login/oauth";
import LoginPasswordFormFragment from "../../components/login/login_password_form_fragment";
import Footer from "../../components/login/footer";

/** @typedef {Object} props
  * @property {Function} totpMode - go to next screen
*/

/** @param {props} props */
const LoginPasswordForm = (props) => {
  return (
    <>
      <Headers />
      <Oauth2 />
      <LoginPasswordFormFragment totpMode={props.totpMode} />
      <Footer />
      </>
  );
};

export default LoginPasswordForm;
