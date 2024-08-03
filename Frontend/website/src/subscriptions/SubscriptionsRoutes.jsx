import { Route } from '@solidjs/router';

import Main from './pages/Main';

let SubscriptionsRoutes = () => {
  return (
    <>
      <Route path="/subscriptions" component={Main} />
    </>
  );
};

export default SubscriptionsRoutes;
