import { lazy } from 'solid-js';
import { Router, Route } from '@solidjs/router';

const Login = lazy(() => import('./login/Login'));
const Logout = lazy(() => import('./Logout'));
const Register = lazy(() => import('./register/Register'));
const Activate = lazy(() => import('./register/Activate'));
const PasswordReset = lazy(() => import('./password_reset/PasswordReset'));
const PasswordResetToken = lazy(() => import('./password_reset/PasswordResetToken'));

let AccountRoutes = () => {
  return (
    <Router>
      <Route path="/accounts/register/" component={Register} />
      <Route path="/accounts/login/" component={Login} />
      <Route path="/accounts/logout/" component={Logout} />
      <Route path="/accounts/activate/:uidb64/:token" component={Activate} />
      <Route path="/accounts/password-reset/" component={PasswordReset} />
      <Route path="/accounts/password-reset/:uidb64/:token" component={PasswordResetToken} />
    </Router>
  )
};

export default AccountRoutes;
