import pulumi
from pulumi_aws import kms, s3

# Create a KMS Key for S3 server-side encryption
key = kms.Key('my-key')

# Create an AWS resource (S3 Bucket)
bucket = s3.Bucket('my-bucket', server_side_encryption_configuration={
        'rule': {
            'apply_server_side_encryption_by_default': {
                'sse_algorithm': 'aws:kms',
                'kms_master_key_id': key.id
            }
        }
    })

# Export the name of the bucket
pulumi.export('bucket_name',  bucket.id)
