// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app"
//import { initializeFirestore, CACHE_SIZE_UNLIMITED, enableIndexedDbPersistence } from "firebase/firestore"
import { getAnalytics } from "firebase/analytics"
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
const firebaseConfig = {
    apiKey: "AIzaSyCEidKgANiGLp_egyOUppyMv06F84UoLVM",
    authDomain: "flashi-13c9b.firebaseapp.com",
    databaseURL: "https://flashi-13c9b-default-rtdb.europe-west1.firebasedatabase.app",
    projectId: "flashi-13c9b",
    storageBucket: "flashi-13c9b.appspot.com",
    messagingSenderId: "104229536375",
    appId: "1:104229536375:web:3f9584fe4aa8f40e40bc72",
}

// Initialize Firebase
const app = initializeApp(firebaseConfig)
getAnalytics(app)
//enableIndexedDbPersistence(initializeFirestore(app, { cacheSizeBytes: CACHE_SIZE_UNLIMITED }))
