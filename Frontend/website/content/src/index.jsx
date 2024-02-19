// import { lazy } from 'solid-js';
// import { render } from 'solid-js/web';
// import { Router, Route } from '@solidjs/router';
//
// const Login = lazy(() => import('./accounts/Login'));
// const LoginTotp = lazy(() => import('./accounts/LoginTotp'));
// const Logout = lazy(() => import('./accounts/Logout'));
// const Register = lazy(() => import('./accounts/Register'));
// const Activate = lazy(() => import('./accounts/Activate'));
// const UsernameReset = lazy(() => import('./accounts/UsernameReset'));
// const UsernameResetToken = lazy(() => import('./accounts/UsernameResetToken'));
// const PasswordReset = lazy(() => import('./accounts/PasswordReset'));
// const PasswordResetToken = lazy(() => import('./accounts/PasswordResetToken'));
//
// /** @typedef { import('solid-js/web').MountableElement } MountableElement */
// render(() => (
//   <Router>
//     <Route path="/accounts/login/" component={Login} />
//     <Route path="/accounts/login/2fa" component={LoginTotp} />
//     <Route path="/accounts/logout/" component={Logout} />
//     <Route path="/accounts/register/" component={Register} />
//     <Route path="/accounts/activate/:uidb64/:token" component={Activate} />
//     <Route path="/accounts/username-reset/" component={UsernameReset} />
//     <Route path="/accounts/username-reset/:uidb64/:token" component={UsernameResetToken} />
//     <Route path="/accounts/password-reset/" component={PasswordReset} />
//     <Route path="/accounts/password-reset/:uidb64/:token" component={PasswordResetToken} />
//   </Router>
// ), (/** @type MountableElement */ (document.getElementById('root'))) );
import { render } from 'solid-js/web';

import AccountRoutes from './accounts/AccountRoutes';
import SettingsRoutes from './settings/SettingsRoutes';

/** @typedef { import('solid-js/web').MountableElement } MountableElement */
render(() => (
  <>
    <AccountRoutes />
    <SettingsRoutes />
  </>
), (/** @type MountableElement */ (document.getElementById('root'))) );
