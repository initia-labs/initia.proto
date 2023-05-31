import com.google.protobuf.gradle.*

plugins {
    kotlin("jvm")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = JavaVersion.VERSION_11.toString()
}

protobuf {
    plugins {
        val grpcKotlinVersion: String by project

        id("grpcKotlin") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:$grpcKotlinVersion:jdk8@jar"
        }
    }

    generateProtoTasks {
        all().forEach { task ->
            task.builtins {
                remove(id = "java")
            }

            task.plugins {
                id("grpcKotlin") {
                    outputSubDir = "kotlin"
                }
            }
        }
    }
}

dependencies {
    val grpcKotlinVersion: String by project

    api(project(":"))
    api(project(":proto-kotlin"))

    api(project(":grpc-java"))
    api("io.grpc:grpc-kotlin-stub:$grpcKotlinVersion")

    compileOnly("javax.annotation:javax.annotation-api:1.3.2")
}

tasks.create("cleanProto") {
    group = "other"

    doLast {
        delete("${project.projectDir.absolutePath}/src/main/kotlin")
    }

    finalizedBy(tasks.getByName("clean"))
}
