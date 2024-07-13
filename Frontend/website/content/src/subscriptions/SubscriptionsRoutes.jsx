import { Route } from '@solidjs/router';

import Subscriptions from './pages/Subscriptions';


let SubscriptionsRoutes = () => {
  return (
    <>
      <Route path="/subscriptions" component={Subscriptions} />
    </>
  )
};

export default SubscriptionsRoutes;

