import { lazy } from 'solid-js';
import { Router, Route } from '@solidjs/router';

const Login = lazy(() => import('./Login'));
const LoginTotp = lazy(() => import('./LoginTotp'));
const Logout = lazy(() => import('./Logout'));
const Register = lazy(() => import('./Register'));
const Activate = lazy(() => import('./Activate'));
const UsernameReset = lazy(() => import('./UsernameReset'));
const UsernameResetToken = lazy(() => import('./UsernameResetToken'));
const PasswordReset = lazy(() => import('./PasswordReset'));
const PasswordResetToken = lazy(() => import('./PasswordResetToken'));

let AccountRoutes = () => {
  return (
    <Router>
      <Route path="/accounts/login/" component={Login} />
      <Route path="/accounts/login/2fa" component={LoginTotp} />
      <Route path="/accounts/logout/" component={Logout} />
      <Route path="/accounts/register/" component={Register} />
      <Route path="/accounts/activate/:uidb64/:token" component={Activate} />
      <Route path="/accounts/username-reset/" component={UsernameReset} />
      <Route path="/accounts/username-reset/:uidb64/:token" component={UsernameResetToken} />
      <Route path="/accounts/password-reset/" component={PasswordReset} />
      <Route path="/accounts/password-reset/:uidb64/:token" component={PasswordResetToken} />
    </Router>
  )
};

export default AccountRoutes;
