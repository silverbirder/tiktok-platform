```
$ git clone https://github.com/tfutils/tfenv.git ~/.tfenv
$ echo 'export PATH="$HOME/.tfenv/bin:$PATH"' >> ~/.bash_profile
$ source ~/.bash_profile
$ tfenv install
$ cd dev
dev $ export PROJECT_ID=$(gcloud config get-value project)
dev $ export BUCKET_NAME=${PROJECT_ID}-tfstate
dev $ export LOCATION=asia-northeast1
dev $ export GOOGLE_PROJECT=$PROJECT_ID
dev $ gcloud auth application-default login
dev $ gsutil mb -p $PROJECT_ID -l $LOCATION gs://$BUCKET_NAME
dev $ gsutil versioning set on gs://$BUCKET_NAME
dev $ cp dev.tfbackend.sample dev.tfbackend
dev $ ln -s ../shared/shared-variables.tf.sample shared-variables.tf
dev $ terraform init -reconfigure -backend-config=dev.tfbackend
dev $ terraform import google_storage_bucket.tfstate $BUCKET_NAME
```