// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app"
import { getAnalytics } from "firebase/analytics"

//FOR emulate
import { getAuth, connectAuthEmulator } from "firebase/auth"
import { getDatabase, connectDatabaseEmulator } from "firebase/database"
import { getFirestore, connectFirestoreEmulator } from "firebase/firestore"
import { getStorage, connectStorageEmulator } from "firebase/storage"
import { getFunctions, connectFunctionsEmulator } from "firebase/functions"

// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
const firebaseConfig = {
    apiKey: import.meta.env.VITE_FIREBASE_API_KEY,
    authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN,
    databaseURL: import.meta.env.VITE_FIREBASE_DATABASE_URL,
    projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID,
    storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET,
    messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID,
    appId: import.meta.env.VITE_FIREBASE_APP_ID,
}

// Initialize Firebase
const app = initializeApp(firebaseConfig)
getAnalytics(app)

if (import.meta.env.DEV && import.meta.env.VITE_FIREBASE_EMULATOR === "true") {
    const auth = getAuth()
    const db = getDatabase()
    const firestore = getFirestore()
    const storage = getStorage()
    const functions = getFunctions(app)

    connectAuthEmulator(auth, "http://localhost:9099")
    connectDatabaseEmulator(db, "localhost", 9000)
    connectFirestoreEmulator(firestore, "localhost", 8080)
    connectStorageEmulator(storage, "localhost", 9199)
    connectFunctionsEmulator(functions, "localhost", 5001)
}
