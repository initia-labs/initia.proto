import com.google.protobuf.gradle.*

plugins {
    kotlin("jvm")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = JavaVersion.VERSION_11.toString()
}

kotlin {
    sourceSets {
        all {
            languageSettings {
                optIn("kotlin.RequiresOptIn")
            }
        }
    }
}

protobuf {
    generateProtoTasks {
        all().forEach { task ->
            task.builtins {
                id("kotlin")

                remove(id = "java")
            }
        }
    }
}

dependencies {
    val protobufVersion: String by project

    api(project(":"))
    api("com.google.protobuf:protobuf-kotlin:$protobufVersion")

    compileOnly("javax.annotation:javax.annotation-api:1.3.2")
}

tasks.create("cleanProto") {
    group = "other"

    doLast {
        delete("${project.projectDir.absolutePath}/src/main/kotlin")
    }

    finalizedBy(tasks.getByName("clean"))
}
