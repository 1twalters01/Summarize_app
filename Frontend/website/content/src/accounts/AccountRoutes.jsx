import { Route } from '@solidjs/router';
import Register from './register/Register';
import Login from './login/Login';
import Logout from './Logout';
import Activate from './register/Activate';
import PasswordReset from './password_reset/PasswordReset';
import PasswordResetToken from './password_reset/PasswordResetToken';

// import { lazy } from 'solid-js';
// const Login = lazy(() => import('./login/Login'));
// const Logout = lazy(() => import('./Logout'));
// const Register = lazy(() => import('./register/Register'));
// const Activate = lazy(() => import('./register/Activate'));
// const PasswordReset = lazy(() => import('./password_reset/PasswordReset'));
// const PasswordResetToken = lazy(() => import('./password_reset/PasswordResetToken'));

let AccountRoutes = () => {
  return (
    <>
      <Route path="register/" component={Register} />
      <Route path="activate/:uidb64/:token/" component={Activate} />
      <Route path="login/" component={Login} />
      <Route path="password-reset/" component={PasswordReset} />
      <Route path="password-reset/:uidb64/:token/" component={PasswordResetToken} />
      <Route path="logout/" component={Logout} />
    </>
  )
};

export default AccountRoutes;
