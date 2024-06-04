/** @param {String} name
  * @param {String} value
  * @param {number} minutesToExpire
*/
export function setCookie(name, value, minutesToExpire) {
  let expires = "";
  if (minutesToExpire) {
    const date = new Date();
    date.setTime(date.getTime() + (minutesToExpire * 60 * 1000));
    expires = "; expires=" + date.toUTCString();
  }
  document.cookie = name + "=" + value + expires + "; path=/; secure; SameSite=Strict";
}

/** @param {String} name */
export function getCookie(name) {
  const cookieString = document.cookie;
  const cookies = cookieString.split("; ");
  
  for (let i=0; i<cookies.length; i++) {
    const cookie = cookies[i].split("=");
    const cookieName = cookie[0];
    if (cookieName === name) {
      const cookieValue = cookie[1];
      return decodeURIComponent(cookieValue);
    }
  }
  return null;
}

/** @param {String} name */
export function deleteCookie(name) {
  const date = new Date();
  date.setTime(date.getTime() - 1000*24*60*60*1000);
  let expires = "; expires=" + date.toUTCString();
  document.cookie = name + "=" + expires + "; path=/; secure; SameSite=Strict";
}

