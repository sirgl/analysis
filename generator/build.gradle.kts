
plugins {
    kotlin("jvm") version "1.3.10"
}

group = "sirgl"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    compile(kotlin("stdlib-jdk8"))
    testCompile("junit", "junit", "4.12")
}

configure<JavaPluginConvention> {

    sourceCompatibility = JavaVersion.VERSION_1_8
}