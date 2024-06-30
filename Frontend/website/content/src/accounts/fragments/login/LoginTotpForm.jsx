import Headers from "../../components/login/headers";
import Oauth2 from "../../components/login/oauth";
import LoginTotpFormFragment from "../../components/login/login_totp_form_fragment";
import Footer from "../../components/login/footer";

/** @typedef {Object} props
  * @property {Function} totpMode - go to next screen
*/

/** @param {props} props */
const LoginTotpForm = (props) => {
  return (
    <>
      <Headers />
      <Oauth2 />
      <LoginTotpFormFragment />
      <Footer />
      </>
  );
};

export default LoginTotpForm;
