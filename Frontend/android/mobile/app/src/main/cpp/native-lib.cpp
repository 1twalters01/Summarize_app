#include <jni.h>
#include <string>

extern "C" JNIEXPORT jstring

JNICALL
Java_uk_summarize_summarize_MainActivity_stringFromJNI(
        JNIEnv *env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}

extern "C" JNIEXPORT jstring

JNICALL
Java_uk_summarize_summarize_MainActivity_prevBtnFromJNI(
        JNIEnv *env,
        jobject /* this */) {
    std::string hello = "Previous from C++";
    return env->NewStringUTF(hello.c_str());
}

extern "C" JNIEXPORT jstring

JNICALL
Java_uk_summarize_summarize_MainActivity_nextBtnFromJNI(
        JNIEnv *env,
        jobject /* this */) {
    std::string hello = "Next from C++";
    return env->NewStringUTF(hello.c_str());
}