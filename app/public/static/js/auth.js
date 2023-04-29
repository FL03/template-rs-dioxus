import { auth } from 'firebase/app';
import firebaseui from 'firebaseui';

var ui = new firebaseui.auth.AuthUI(auth());
// Initialize the FirebaseUI Widget using Firebase.
var uiConfig = {
    callbacks: {
      signInSuccessWithAuthResult: function(authResult, redirectUrl) {
        // User successfully signed in.
        // Return type determines whether we continue the redirect automatically
        // or whether we leave that to developer to handle.
        return true;
      },
      uiShown: function() {
        // The widget is rendered; hide the loader.
        document.getElementById('load').classList.add('hidden');
      }
    },
    // Will use popup for IDP Providers sign-in flow instead of the default, redirect.
    signInFlow: 'popup',
    signInSuccessUrl: '<url-to-redirect-to-on-success>',
    signInOptions: [
      // Leave the lines as is for the providers you want to offer your users.
      auth.GoogleAuthProvider.PROVIDER_ID,
      auth.GithubAuthProvider.PROVIDER_ID,
      auth.EmailAuthProvider.PROVIDER_ID,
      auth.PhoneAuthProvider.PROVIDER_ID
    ],
    // Terms of service url.
    tosUrl: '<your-tos-url>',
    // Privacy policy url.
    privacyPolicyUrl: '<your-privacy-policy-url>'
};

// Start the FirebaseUI Auth UI
ui.start('#firebaseui-auth-container', uiConfig);
