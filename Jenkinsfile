pipeline {
    agent {
        dockerfile {
            filename 'infra/docker/Dockerfile'
            label 'docker'
        }
    }
    options {
        disableConcurrentBuilds()
        timeout(time: 60, unit: 'MINUTES')
    }
    environment {
        HOME = "${WORKSPACE}/.cache/home"
        CARGO_HOME = "${WORKSPACE}/.cache/cargo"
        RUSTUP_HOME = '/usr/local/rustup'
    }
    stages {
        stage('Tests') {
            steps {
                sh 'mkdir -p "$HOME" "$CARGO_HOME"'
                sh 'npm test'
                sh 'cargo test --workspace --locked'
                sh 'cargo fmt --all -- --check'
            }
        }
        stage('Web') {
            steps {
                sh 'npm run build'
            }
        }
        stage('Desktop') {
            steps {
                sh 'cargo build -p dedale-desktop --release --locked'
            }
        }
        stage('Android') {
            steps {
                sh 'bash scripts/build-android.sh release'
            }
        }
    }
    post {
        success {
            archiveArtifacts artifacts: 'apps/web/dist/**,target/release/dedale-desktop,target/android-artifacts/release/apk/*.apk',
                fingerprint: true
        }
    }
}
