# Kubeprovision

Provision kubernetes cluster nodes from scratch.

## Usage

```shell
# check target instances
kubeprovision status

# start ec2 instances
kubeprovision start

# provision
kubeprovision provision

# stop ec2 instances
kubeprovision stop
```

## Target EC2 Instances(Nodes)

Tags are used to identify the EC2 instance to be provisioned.  
In the case of `config/example.yaml`, all instances must have `example:project`=`handson` tag.  
The master/worker distinction is made by specifying the `aws.ec2.node.(master|worker)` tag setting in yaml.
