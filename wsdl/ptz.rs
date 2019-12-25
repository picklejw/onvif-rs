<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="../../../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2014 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" name="PTZService" targetNamespace="http://www.onvif.org/ver20/ptz/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver20/ptz/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="17.06">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>
			<!--  Message Request/Responses elements  -->
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tptz:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the PTZ service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="EFlip" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether or not EFlip is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Reverse" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether or not reversing of PT control direction is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="GetCompatibleConfigurations" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for the GetCompatibleConfigurations command.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MoveStatus" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates that the PTZStatus includes MoveStatus information.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="StatusPosition" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates that the PTZStatus includes Position information.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="tptz:Capabilities"/>
			<!--===============================-->
			<xs:element name="GetNodes">
				<xs:complexType/>
			</xs:element>
			<xs:element name="GetNodesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZNode" type="tt:PTZNode" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>A list of the existing PTZ Nodes on the device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NodeToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested PTZNode.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNodeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZNode" type="tt:PTZNode">
							<xs:annotation>
								<xs:documentation>A requested PTZNode.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetConfigurations">
				<xs:complexType/>
			</xs:element>
			<xs:element name="GetConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfiguration" type="tt:PTZConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>A list of all existing PTZConfigurations on the device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested PTZConfiguration.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfiguration" type="tt:PTZConfiguration">
							<xs:annotation>
								<xs:documentation>A requested PTZConfiguration.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfiguration" type="tt:PTZConfiguration">
							<xs:annotation>
								<xs:documentation>
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Flag that makes configuration persistent. Example: User wants the configuration to exist after reboot.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetConfigurationResponse">
				<xs:complexType>
					<xs:sequence minOccurs="0"/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of an existing configuration that the options are intended for.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfigurationOptions" type="tt:PTZConfigurationOptions">
							<xs:annotation>
								<xs:documentation>The requested PTZ configuration options.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SendAuxiliaryCommand">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="AuxiliaryData" type="tt:AuxiliaryData">
							<xs:annotation>
								<xs:documentation>The Auxiliary request data.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SendAuxiliaryCommandResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AuxiliaryResponse" type="tt:AuxiliaryData">
							<xs:annotation>
								<xs:documentation>The response contains the auxiliary response.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPresets">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPresetsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Preset" type="tt:PTZPreset" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>A list of presets which are available for the requested MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetPreset">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PresetName" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>A requested preset name.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PresetToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>A requested preset token.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetPresetResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PresetToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A token to the Preset which has been set.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemovePreset">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PresetToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A requested preset token.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemovePresetResponse">
				<xs:complexType/>
			</xs:element>
			<!--===============================-->
			<xs:element name="GotoPreset">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PresetToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A requested preset token.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Speed" type="tt:PTZSpeed" minOccurs="0">
							<xs:annotation>
								<xs:documentation>A requested speed.The speed parameter can only be specified when Speed Spaces are available for the PTZ Node.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GotoPresetResponse">
				<xs:complexType/>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetStatus">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the PTZStatus should be requested.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetStatusResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZStatus" type="tt:PTZStatus">
							<xs:annotation>
								<xs:documentation>The PTZStatus for the requested MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GotoHomePosition">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the operation should take place.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Speed" type="tt:PTZSpeed" minOccurs="0">
							<xs:annotation>
								<xs:documentation>A requested speed.The speed parameter can only be specified when Speed Spaces are available for the PTZ Node.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GotoHomePositionResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetHomePosition">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile where the home position should be set.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetHomePositionResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="ContinuousMove">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Velocity" type="tt:PTZSpeed">
							<xs:annotation>
								<xs:documentation>A Velocity vector specifying the velocity of pan, tilt and zoom.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Timeout" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional Timeout parameter.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ContinuousMoveResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RelativeMove">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Translation" type="tt:PTZVector">
							<xs:annotation>
								<xs:documentation>A positional Translation relative to the current position
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Speed" type="tt:PTZSpeed" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional Speed parameter.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RelativeMoveResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AbsoluteMove">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Position" type="tt:PTZVector">
							<xs:annotation>
								<xs:documentation>A Position vector specifying the absolute target position.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Speed" type="tt:PTZSpeed" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional Speed.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AbsoluteMoveResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GeoMove">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Target" type="tt:GeoLocation">
							<xs:annotation>
								<xs:documentation>The geolocation of the target position.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Speed" type="tt:PTZSpeed" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional Speed.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="AreaHeight" type="xs:float" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional indication of the height of the target/area.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="AreaWidth" type="xs:float" minOccurs="0">
							<xs:annotation>
								<xs:documentation>An optional indication of the width of the target/area.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GeoMoveResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="Stop">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>A reference to the MediaProfile that indicate what should be stopped.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PanTilt" type="xs:boolean" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Set true when we want to stop ongoing pan and tilt movements.If PanTilt arguments are not present, this command stops these movements.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Zoom" type="xs:boolean" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Set true when we want to stop ongoing zoom movement.If Zoom arguments are not present, this command stops ongoing zoom movement.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="StopResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPresetTours">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPresetToursResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PresetTour" type="tt:PresetTour" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPresetTour">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
						<xs:element name="PresetTourToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPresetTourResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PresetTour" type="tt:PresetTour"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPresetTourOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
						<xs:element name="PresetTourToken" type="tt:ReferenceToken" minOccurs="0"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPresetTourOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:PTZPresetTourOptions"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreatePresetTour">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreatePresetTourResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PresetTourToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="ModifyPresetTour">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
						<xs:element name="PresetTour" type="tt:PresetTour"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ModifyPresetTourResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="OperatePresetTour">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
						<xs:element name="PresetTourToken" type="tt:ReferenceToken"/>
						<xs:element name="Operation" type="tt:PTZPresetTourOperation"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="OperatePresetTourResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemovePresetTour">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken"/>
						<xs:element name="PresetTourToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemovePresetTourResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PTZConfiguration" type="tt:PTZConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>A list of all existing PTZConfigurations on the NVT that is suitable to be added to the addressed media profile.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tptz:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tptz:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNodesRequest">
		<wsdl:part name="parameters" element="tptz:GetNodes"/>
	</wsdl:message>
	<wsdl:message name="GetNodesResponse">
		<wsdl:part name="parameters" element="tptz:GetNodesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNodeRequest">
		<wsdl:part name="parameters" element="tptz:GetNode"/>
	</wsdl:message>
	<wsdl:message name="GetNodeResponse">
		<wsdl:part name="parameters" element="tptz:GetNodeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationsRequest">
		<wsdl:part name="parameters" element="tptz:GetConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationsResponse">
		<wsdl:part name="parameters" element="tptz:GetConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationRequest">
		<wsdl:part name="parameters" element="tptz:GetConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationResponse">
		<wsdl:part name="parameters" element="tptz:GetConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetConfigurationRequest">
		<wsdl:part name="parameters" element="tptz:SetConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetConfigurationResponse">
		<wsdl:part name="parameters" element="tptz:SetConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tptz:GetConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tptz:GetConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPresetsRequest">
		<wsdl:part name="parameters" element="tptz:GetPresets"/>
	</wsdl:message>
	<wsdl:message name="GetPresetsResponse">
		<wsdl:part name="parameters" element="tptz:GetPresetsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetPresetRequest">
		<wsdl:part name="parameters" element="tptz:SetPreset"/>
	</wsdl:message>
	<wsdl:message name="SetPresetResponse">
		<wsdl:part name="parameters" element="tptz:SetPresetResponse"/>
	</wsdl:message>
	<wsdl:message name="RemovePresetRequest">
		<wsdl:part name="parameters" element="tptz:RemovePreset"/>
	</wsdl:message>
	<wsdl:message name="RemovePresetResponse">
		<wsdl:part name="parameters" element="tptz:RemovePresetResponse"/>
	</wsdl:message>
	<wsdl:message name="GotoPresetRequest">
		<wsdl:part name="parameters" element="tptz:GotoPreset"/>
	</wsdl:message>
	<wsdl:message name="GotoPresetResponse">
		<wsdl:part name="parameters" element="tptz:GotoPresetResponse"/>
	</wsdl:message>
	<wsdl:message name="GetStatusRequest">
		<wsdl:part name="parameters" element="tptz:GetStatus"/>
	</wsdl:message>
	<wsdl:message name="GetStatusResponse">
		<wsdl:part name="parameters" element="tptz:GetStatusResponse"/>
	</wsdl:message>
	<wsdl:message name="SendAuxiliaryCommandRequest">
		<wsdl:part name="parameters" element="tptz:SendAuxiliaryCommand"/>
	</wsdl:message>
	<wsdl:message name="SendAuxiliaryCommandResponse">
		<wsdl:part name="parameters" element="tptz:SendAuxiliaryCommandResponse"/>
	</wsdl:message>
	<wsdl:message name="GotoHomePositionRequest">
		<wsdl:part name="parameters" element="tptz:GotoHomePosition"/>
	</wsdl:message>
	<wsdl:message name="GotoHomePositionResponse">
		<wsdl:part name="parameters" element="tptz:GotoHomePositionResponse"/>
	</wsdl:message>
	<wsdl:message name="SetHomePositionRequest">
		<wsdl:part name="parameters" element="tptz:SetHomePosition"/>
	</wsdl:message>
	<wsdl:message name="SetHomePositionResponse">
		<wsdl:part name="parameters" element="tptz:SetHomePositionResponse"/>
	</wsdl:message>
	<wsdl:message name="ContinuousMoveRequest">
		<wsdl:part name="parameters" element="tptz:ContinuousMove"/>
	</wsdl:message>
	<wsdl:message name="ContinuousMoveResponse">
		<wsdl:part name="parameters" element="tptz:ContinuousMoveResponse"/>
	</wsdl:message>
	<wsdl:message name="RelativeMoveRequest">
		<wsdl:part name="parameters" element="tptz:RelativeMove"/>
	</wsdl:message>
	<wsdl:message name="RelativeMoveResponse">
		<wsdl:part name="parameters" element="tptz:RelativeMoveResponse"/>
	</wsdl:message>
	<wsdl:message name="AbsoluteMoveRequest">
		<wsdl:part name="parameters" element="tptz:AbsoluteMove"/>
	</wsdl:message>
	<wsdl:message name="AbsoluteMoveResponse">
		<wsdl:part name="parameters" element="tptz:AbsoluteMoveResponse"/>
	</wsdl:message>
	<wsdl:message name="GeoMoveRequest">
		<wsdl:part name="parameters" element="tptz:GeoMove"/>
	</wsdl:message>
	<wsdl:message name="GeoMoveResponse">
		<wsdl:part name="parameters" element="tptz:GeoMoveResponse"/>
	</wsdl:message>
	<wsdl:message name="StopRequest">
		<wsdl:part name="parameters" element="tptz:Stop"/>
	</wsdl:message>
	<wsdl:message name="StopResponse">
		<wsdl:part name="parameters" element="tptz:StopResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPresetToursRequest">
		<wsdl:part name="parameters" element="tptz:GetPresetTours"/>
	</wsdl:message>
	<wsdl:message name="GetPresetToursResponse">
		<wsdl:part name="parameters" element="tptz:GetPresetToursResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPresetTourRequest">
		<wsdl:part name="parameters" element="tptz:GetPresetTour"/>
	</wsdl:message>
	<wsdl:message name="GetPresetTourResponse">
		<wsdl:part name="parameters" element="tptz:GetPresetTourResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPresetTourOptionsRequest">
		<wsdl:part name="parameters" element="tptz:GetPresetTourOptions"/>
	</wsdl:message>
	<wsdl:message name="GetPresetTourOptionsResponse">
		<wsdl:part name="parameters" element="tptz:GetPresetTourOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="CreatePresetTourRequest">
		<wsdl:part name="parameters" element="tptz:CreatePresetTour"/>
	</wsdl:message>
	<wsdl:message name="CreatePresetTourResponse">
		<wsdl:part name="parameters" element="tptz:CreatePresetTourResponse"/>
	</wsdl:message>
	<wsdl:message name="ModifyPresetTourRequest">
		<wsdl:part name="parameters" element="tptz:ModifyPresetTour"/>
	</wsdl:message>
	<wsdl:message name="ModifyPresetTourResponse">
		<wsdl:part name="parameters" element="tptz:ModifyPresetTourResponse"/>
	</wsdl:message>
	<wsdl:message name="OperatePresetTourRequest">
		<wsdl:part name="parameters" element="tptz:OperatePresetTour"/>
	</wsdl:message>
	<wsdl:message name="OperatePresetTourResponse">
		<wsdl:part name="parameters" element="tptz:OperatePresetTourResponse"/>
	</wsdl:message>
	<wsdl:message name="RemovePresetTourRequest">
		<wsdl:part name="parameters" element="tptz:RemovePresetTour"/>
	</wsdl:message>
	<wsdl:message name="RemovePresetTourResponse">
		<wsdl:part name="parameters" element="tptz:RemovePresetTourResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleConfigurationsRequest">
		<wsdl:part name="parameters" element="tptz:GetCompatibleConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleConfigurationsResponse">
		<wsdl:part name="parameters" element="tptz:GetCompatibleConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:portType name="PTZ">
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the PTZ service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tptz:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tptz:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNodes">
			<wsdl:documentation>
				Get the descriptions of the available PTZ Nodes.
				<br/>
				A PTZ-capable device may have multiple PTZ Nodes. The PTZ Nodes may represent
				mechanical PTZ drivers, uploaded PTZ drivers or digital PTZ drivers. PTZ Nodes are the
				lowest level entities in the PTZ control API and reflect the supported PTZ capabilities. The
				PTZ Node is referenced either by its name or by its reference token. 
      		</wsdl:documentation>
			<wsdl:input message="tptz:GetNodesRequest"/>
			<wsdl:output message="tptz:GetNodesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNode">
			<wsdl:documentation>Get a specific PTZ Node identified by a reference
        token or a name.
	  </wsdl:documentation>
			<wsdl:input message="tptz:GetNodeRequest"/>
			<wsdl:output message="tptz:GetNodeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetConfiguration">
			<wsdl:documentation>Get a specific PTZconfiguration from the device, identified by its reference token or name.
				<br/>
				The default Position/Translation/Velocity Spaces are introduced to allow NVCs sending move
				requests without the need to specify a certain coordinate system. The default Speeds are
				introduced to control the speed of move requests (absolute, relative, preset), where no
				explicit speed has been set.<br/>
				The allowed pan and tilt range for Pan/Tilt Limits is defined by a two-dimensional space range
				that is mapped to a specific Absolute Pan/Tilt Position Space. At least one Pan/Tilt Position
				Space is required by the PTZNode to support Pan/Tilt limits. The limits apply to all supported
				absolute, relative and continuous Pan/Tilt movements. The limits shall be checked within the
				coordinate system for which the limits have been specified. That means that even if
				movements are specified in a different coordinate system, the requested movements shall be
				transformed to the coordinate system of the limits where the limits can be checked. When a
				relative or continuous movements is specified, which would leave the specified limits, the PTZ
				unit has to move along the specified limits. The Zoom Limits have to be interpreted
				accordingly.
			</wsdl:documentation>
			<wsdl:input message="tptz:GetConfigurationRequest"/>
			<wsdl:output message="tptz:GetConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetConfigurations">
			<wsdl:documentation>
		        Get all the existing PTZConfigurations from the device.
				<br/>
				The default Position/Translation/Velocity Spaces are introduced to allow NVCs sending move
				requests without the need to specify a certain coordinate system. The default Speeds are
				introduced to control the speed of move requests (absolute, relative, preset), where no
				explicit speed has been set.<br/>
				The allowed pan and tilt range for Pan/Tilt Limits is defined by a two-dimensional space range
				that is mapped to a specific Absolute Pan/Tilt Position Space. At least one Pan/Tilt Position
				Space is required by the PTZNode to support Pan/Tilt limits. The limits apply to all supported
				absolute, relative and continuous Pan/Tilt movements. The limits shall be checked within the
				coordinate system for which the limits have been specified. That means that even if
				movements are specified in a different coordinate system, the requested movements shall be
				transformed to the coordinate system of the limits where the limits can be checked. When a
				relative or continuous movements is specified, which would leave the specified limits, the PTZ
				unit has to move along the specified limits. The Zoom Limits have to be interpreted
				accordingly.
			</wsdl:documentation>
			<wsdl:input message="tptz:GetConfigurationsRequest"/>
			<wsdl:output message="tptz:GetConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetConfiguration">
			<wsdl:documentation>
        Set/update a existing PTZConfiguration on the device.
      </wsdl:documentation>
			<wsdl:input message="tptz:SetConfigurationRequest"/>
			<wsdl:output message="tptz:SetConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetConfigurationOptions">
			<wsdl:documentation>
				List supported coordinate systems including their range limitations. Therefore, the options
				MAY differ depending on whether the PTZ Configuration is assigned to a Profile containing a
				Video Source Configuration. In that case, the options may additionally contain coordinate
				systems referring to the image coordinate system described by the Video Source
				Configuration. If the PTZ Node supports continuous movements, it shall return a Timeout Range within
				which Timeouts are accepted by the PTZ Node.				
			</wsdl:documentation>
			<wsdl:input message="tptz:GetConfigurationOptionsRequest"/>
			<wsdl:output message="tptz:GetConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SendAuxiliaryCommand">
			<wsdl:documentation>
        Operation to send auxiliary commands to the PTZ device
        mapped by the PTZNode in the selected profile. The
        operation is supported
        if the AuxiliarySupported element of the PTZNode is true
      </wsdl:documentation>
			<wsdl:input message="tptz:SendAuxiliaryCommandRequest"/>
			<wsdl:output message="tptz:SendAuxiliaryCommandResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPresets">
			<wsdl:documentation>
        Operation to request all PTZ presets for the PTZNode
        in the selected profile. The operation is supported if there is support
        for at least on PTZ preset by the PTZNode.</wsdl:documentation>
			<wsdl:input message="tptz:GetPresetsRequest"/>
			<wsdl:output message="tptz:GetPresetsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetPreset">
			<wsdl:documentation>
				The SetPreset command saves the current device position parameters so that the device can
				move to the saved preset position through the GotoPreset operation.
				In order to create a new preset, the SetPresetRequest contains no PresetToken. If creation is
				successful, the Response contains the PresetToken which uniquely identifies the Preset. An
				existing Preset can be overwritten by specifying the PresetToken of the corresponding Preset.
				In both cases (overwriting or creation) an optional PresetName can be specified. The
				operation fails if the PTZ device is moving during the SetPreset operation.
				The device MAY internally save additional states such as imaging properties in the PTZ
				Preset which then should be recalled in the GotoPreset operation.      </wsdl:documentation>
			<wsdl:input message="tptz:SetPresetRequest"/>
			<wsdl:output message="tptz:SetPresetResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemovePreset">
			<wsdl:documentation>
        Operation to remove a PTZ preset for the Node in
        the
        selected profile. The operation is supported if the
        PresetPosition
        capability exists for teh Node in the
        selected profile.
      </wsdl:documentation>
			<wsdl:input message="tptz:RemovePresetRequest"/>
			<wsdl:output message="tptz:RemovePresetResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GotoPreset">
			<wsdl:documentation>
        Operation to go to a saved preset position for the
        PTZNode in the selected profile. The operation is supported if there is
        support for at least on PTZ preset by the PTZNode.</wsdl:documentation>
			<wsdl:input message="tptz:GotoPresetRequest"/>
			<wsdl:output message="tptz:GotoPresetResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GotoHomePosition">
			<wsdl:documentation>
        Operation to move the PTZ device to it's &quot;home&quot; position. The operation is supported if the HomeSupported element in the PTZNode is true.</wsdl:documentation>
			<wsdl:input message="tptz:GotoHomePositionRequest"/>
			<wsdl:output message="tptz:GotoHomePositionResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetHomePosition">
			<wsdl:documentation>Operation to save current position as the home position.
				The SetHomePosition command returns with a failure if the “home” position is fixed and
				cannot be overwritten. If the SetHomePosition is successful, it is possible to recall the
				Home Position with the GotoHomePosition command.</wsdl:documentation>
			<wsdl:input message="tptz:SetHomePositionRequest"/>
			<wsdl:output message="tptz:SetHomePositionResponse"/>
		</wsdl:operation>
		<wsdl:operation name="ContinuousMove">
			<wsdl:documentation>Operation for continuous Pan/Tilt and Zoom movements. The operation is supported if the PTZNode supports at least one continuous Pan/Tilt or Zoom space. If the space argument is omitted, the default space set by the PTZConfiguration will be used.</wsdl:documentation>
			<wsdl:input message="tptz:ContinuousMoveRequest"/>
			<wsdl:output message="tptz:ContinuousMoveResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RelativeMove">
			<wsdl:documentation>Operation for Relative Pan/Tilt and Zoom Move. The operation is supported if the PTZNode supports at least one relative Pan/Tilt or Zoom space.<br/> 
				The speed argument is optional. If an x/y speed value is given it is up to the device to either use 
				the x value as absolute resoluting speed vector or to map x and y to the component speed. 
				If the speed argument is omitted, the default speed set by the PTZConfiguration will be used.
			</wsdl:documentation>
			<wsdl:input message="tptz:RelativeMoveRequest"/>
			<wsdl:output message="tptz:RelativeMoveResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetStatus">
			<wsdl:documentation>
				Operation to request PTZ status for the Node in the
				selected profile.</wsdl:documentation>
			<wsdl:input message="tptz:GetStatusRequest"/>
			<wsdl:output message="tptz:GetStatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AbsoluteMove">
			<wsdl:documentation>Operation to move pan,tilt or zoom to a absolute destination. <br/>
				The speed argument is optional. If an x/y speed value is given it is up to the device to either use 
				the x value as absolute resoluting speed vector or to map x and y to the component speed. 
				If the speed argument is omitted, the default speed set by the PTZConfiguration will be used.
			</wsdl:documentation>
			<wsdl:input message="tptz:AbsoluteMoveRequest"/>
			<wsdl:output message="tptz:AbsoluteMoveResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GeoMove">
			<wsdl:documentation>Operation to move pan,tilt or zoom to point to a destination based on the geolocation of the target. <br/>
				The speed argument is optional. If an x/y speed value is given it is up to the device to either use 
				the x value as absolute resoluting speed vector or to map x and y to the component speed. 
				If the speed argument is omitted, the default speed set by the PTZConfiguration will be used.
				The area height and area dwidth parameters are optional, they can be used independently and may be used
				by the device to automatically determine the best zoom level to show the target.
			</wsdl:documentation>
			<wsdl:input message="tptz:GeoMoveRequest"/>
			<wsdl:output message="tptz:GeoMoveResponse"/>
		</wsdl:operation>
		<wsdl:operation name="Stop">
			<wsdl:documentation>Operation to stop ongoing pan, tilt and zoom movements of absolute relative and continuous type.
If no stop argument for pan, tilt or zoom is set, the device will stop all ongoing pan, tilt and zoom movements.</wsdl:documentation>
			<wsdl:input message="tptz:StopRequest"/>
			<wsdl:output message="tptz:StopResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTours">
			<wsdl:documentation>Operation to request PTZ preset tours in the selected media profiles.</wsdl:documentation>
			<wsdl:input message="tptz:GetPresetToursRequest"/>
			<wsdl:output message="tptz:GetPresetToursResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTour">
			<wsdl:documentation>Operation to request a specific PTZ preset tour in the selected media profile.</wsdl:documentation>
			<wsdl:input message="tptz:GetPresetTourRequest"/>
			<wsdl:output message="tptz:GetPresetTourResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTourOptions">
			<wsdl:documentation>Operation to request available options to configure PTZ preset tour.</wsdl:documentation>
			<wsdl:input message="tptz:GetPresetTourOptionsRequest"/>
			<wsdl:output message="tptz:GetPresetTourOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreatePresetTour">
			<wsdl:documentation>Operation to create a preset tour for the selected media profile.</wsdl:documentation>
			<wsdl:input message="tptz:CreatePresetTourRequest"/>
			<wsdl:output message="tptz:CreatePresetTourResponse"/>
		</wsdl:operation>
		<wsdl:operation name="ModifyPresetTour">
			<wsdl:documentation>Operation to modify a preset tour for the selected media profile.</wsdl:documentation>
			<wsdl:input message="tptz:ModifyPresetTourRequest"/>
			<wsdl:output message="tptz:ModifyPresetTourResponse"/>
		</wsdl:operation>
		<wsdl:operation name="OperatePresetTour">
			<wsdl:documentation>Operation to perform specific operation on the preset tour in selected media profile.</wsdl:documentation>
			<wsdl:input message="tptz:OperatePresetTourRequest"/>
			<wsdl:output message="tptz:OperatePresetTourResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemovePresetTour">
			<wsdl:documentation>Operation to delete a specific preset tour from the media profile.</wsdl:documentation>
			<wsdl:input message="tptz:RemovePresetTourRequest"/>
			<wsdl:output message="tptz:RemovePresetTourResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleConfigurations">
			<wsdl:documentation>Operation to get all available PTZConfigurations that can be added to the referenced media profile. <br/>
				A device providing more than one PTZConfiguration or more than one VideoSourceConfiguration or which has any other resource
				interdependency between PTZConfiguration entities and other resources listable in a media profile should implement this operation.
				PTZConfiguration entities returned by this operation shall not fail on adding them to the referenced media profile.
			</wsdl:documentation>
			<wsdl:input message="tptz:GetCompatibleConfigurationsRequest"/>
			<wsdl:output message="tptz:GetCompatibleConfigurationsResponse"/>
		</wsdl:operation>
	</wsdl:portType>
	<wsdl:binding name="PTZBinding" type="tptz:PTZ">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPresets">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetPresets"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetPreset">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/SetPreset"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemovePreset">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/RemovePreset"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GotoPreset">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GotoPreset"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetStatus">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetStatus"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNodes">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetNodes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNode">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetNode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/SetConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GotoHomePosition">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GotoHomePosition"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetHomePosition">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/SetHomePosition"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="ContinuousMove">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/ContinuousMove"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RelativeMove">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/RelativeMove"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SendAuxiliaryCommand">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/SendAuxiliaryCommand"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="AbsoluteMove">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/AbsoluteMove"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GeoMove">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GeoMove"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="Stop">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/Stop"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTours">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetPresetTours"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTour">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetPresetTour"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPresetTourOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetPresetTourOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreatePresetTour">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/CreatePresetTour"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="ModifyPresetTour">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/ModifyPresetTour"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="OperatePresetTour">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/OperatePresetTour"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemovePresetTour">
			<soap:operation soapAction="http://www.onvif.org/ver20/ptz/wsdl/RemovePresetTour"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleConfigurations">
			<soap:operation
				soapAction="http://www.onvif.org/ver20/ptz/wsdl/GetCompatibleConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
</wsdl:definitions>
