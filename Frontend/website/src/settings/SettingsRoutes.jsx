import { lazy } from 'solid-js';
import { Route } from '@solidjs/router';

const Profile = lazy(() => import('./profile/Page'));
// const ReadingPreference = lazy(() => import('./personalisation/Page'));
// const Notifications = lazy(() => import('./notifications/Page'));
// const Personalisation = lazy(() => import('./personalisation/Page'));
// const Connectivity = lazy(() => import('./connectivity/Page'));
// const AppSettings = lazy(() => import('./app_settings/Page'));
// const Subscriptions = lazy(() => import('./subscriptions/Page'));
// const SupportAndFeedback = lazy(() => import('./support_and_feedback/Page'));
// const Advanced = lazy(() => import('./advanced/Page'));
// const Miscellaneous = lazy(() => import('./miscellaneous/Page'));

let AccountRoutes = () => {
  return (
    <>
      <Route path="settings/profile" component={Profile} />
    </>
  );
  // <Route path="settings/reading-preference/" component={ReadingPreference} />
  // <Route path="settings/notifications/" component={Notifications} />
  // <Route path="settings/personalisation/" component={Personalisation} />
  // <Route path="settings/connectivity/" component={Connectivity} />
  // <Route path="settings/app-settings/" component={AppSettings} />
  // <Route path="settings/subscriptions/" component={Subscriptions} />
  // <Route path="settings/support-and-feedback/" component={SupportAndFeedback} />
  // <Route path="settings/advanced/" component={Advanced} />
  // <Route path="settings/miscellaneous/" component={Miscellaneous} />
};

export default AccountRoutes;
