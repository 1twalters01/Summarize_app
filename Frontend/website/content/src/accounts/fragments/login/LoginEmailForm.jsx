import Headers from "../../components/login/headers";
import Oauth2 from "../../components/login/oauth";
import LoginEmailFormFragment from "../../components/login/login_email_form_fragment";
import Footer from "../../components/login/footer";

/** @typedef {Object} props
  * @property {Function} passwordMode - go to next screen
*/

/** @param {props} props */
const LoginEmailForm = (props) => {
  return (
    <>
      <Headers />
      <Oauth2 />
      <LoginEmailFormFragment passwordMode={props.passwordMode} />
      <Footer />
      </>
  );
};

export default LoginEmailForm;



